import torch
import triton
import triton.language as tl
from triton.runtime import driver

@triton.jit
def softmax_kernel(inp_ptr, out_ptr, x_stride, y_stride, n_rows, n_cols, BLOCK_SIZE : tl.constexpr, num_stages : tl.constexpr):

    pid = tl.program_id(0)
    num_progs = tl.num_programs(0)

    row_start = pid
    step_rows = num_progs
    for x in tl.range(row_start, n_rows, step_rows, num_stages=num_stages):
        i = inp_ptr + x * x_stride
        o = out_ptr + x * y_stride
        offset = tl.arange(0, BLOCK_SIZE)
        mask = offset < n_cols
        a = tl.load(i + offset, mask = mask, other = -float('inf'))
        a_max = a - tl.max(a, axis = 0)
        exp_a = tl.exp(a_max)
        exp_sum = tl.sum(exp_a)
        b = exp_a / exp_sum
        tl.store(o + offset, b, mask = mask)


device = torch.cuda.current_device()
props = driver.active.utils.get_device_properties(device)
num_stream_multproc = props["multiprocessor_count"]
NUM_REGS = props["max_num_regs"]
SIZE_SMEM = props["max_shared_mem"]
WARP_SIZE = props["warpSize"]

target = triton.runtime.driver.active.get_current_target()

kernels = {}

def softmax(x):
    n_rows, n_cols = x.shape

    BLOCK_SIZE = triton.next_power_of_2(n_cols)

    num_warps = 8
    num_stages = 4 if SIZE_SMEM > 200000 else 2

    y = torch.empty_like(x)
    kernel, num_programs = kernels.get(BLOCK_SIZE, (None,0))
    if kernel is None:
        kernel = softmax_kernel.warmup(x, y, x.stride(0), y.stride(0), n_rows, n_cols, 
                                         BLOCK_SIZE=BLOCK_SIZE, num_stages=num_stages, num_warps=num_warps, grid=(1,))
        kernel._init_handles()
        n_regs = kernel.n_regs
        size_smem = kernel.metadata.shared
        occupancy = NUM_REGS // (WARP_SIZE*num_warps*n_regs)
        occupancy = min(occupancy, SIZE_SMEM/size_smem)
        num_programs = occupancy * num_stream_multproc
        kernels[BLOCK_SIZE] = (kernel, num_programs)
    
    num_programs = min(num_programs, n_rows)

    kernel[(num_programs, 1, 1)](
        x,
        y,
        x.stride(0),
        y.stride(0),
        n_rows, 
        n_cols,
    )
    return y

a = torch.randn(1024, 1024, device = 'cuda')
b_triton = softmax(a)
b_torch = torch.softmax(a, dim = -1)

print(torch.allclose(b_triton, b_torch))
