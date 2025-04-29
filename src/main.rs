mod memory;

use memory::VulnerableBuffer;

fn main() {
    // 작은 버퍼 생성
    let mut buffer = VulnerableBuffer::new(10);
    
    // 버퍼 오버플로우 시도
    let large_data = vec![0x41; 20];
    match buffer.write(&large_data) {
        Ok(_) => println!("Write successful"),
        Err(e) => println!("Write failed: {}", e),
    }
    
    // 데이터 읽기 시도
    match buffer.read(20) {
        Ok(read_data) => println!("Read data: {:?}", read_data),
        Err(e) => println!("Read failed: {}", e),
    }
    
    // 메모리 덤프
    unsafe {
        let ptr = buffer.get_data_ptr();
        for i in 0..10 {  // 버퍼 크기만큼만 읽기
            print!("{:02x} ", *ptr.add(i));
        }
        println!();
    }
}
