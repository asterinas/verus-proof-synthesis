use vstd::prelude::*;
fn main() {}
verus!{
pub fn myfun(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32)
	requires
		N > 0,
		old(a).len() == N,
		old(sum).len() == 1,
	ensures
		forall |k:int| 0 <= k < N ==> a[k] == 0,
{
	sum.set(0, 0);
	let mut i: usize = 0;
	while (i < N as usize)
	{
		sum.set(0, sum[0] + 1);
		i = i + 1;
	}

	i = 0;
	while (i < N as usize)
	{
		a.set(i, sum[0] % N);
		i = i + 1;
	}
}
}