pub fn fibo(n: usize)->Result<Vec<u64>,String>{
    if n > 93 {
        return Err(String::from("Too large—u64 overflow possible!"));
    }

    let mut fib = Vec::new();
    if n==0{
        return Ok(fib);
    }

    fib.push(0);
    if n==1{
        return Ok(fib);
    }

    fib.push(1);
    for i in 2..n{
        let next = fib[i-1]+fib[i-2];
        fib.push(next);
    }
    return Ok(fib) 
}