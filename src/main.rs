mod memory;

use memory::VulnerableBuffer;

fn main() {
    // 작은 버퍼 생성
    let mut buffer = VulnerableBuffer::new(10);
    
    // 버퍼 오버플로우 발생
    let large_data = vec![0x41; 20]; // 'A' 문자 20개
    buffer.write(&large_data); // 10바이트 버퍼에 20바이트 쓰기
    
    // 오버플로우된 데이터 읽기
    let read_data = buffer.read(20);
    println!("Read data: {:?}", read_data);
    
    // 메모리 덤프 (취약점 시연)
    unsafe {
        let ptr = buffer.get_data_ptr();
        for i in 0..20 {
            print!("{:02x} ", *ptr.add(i));
        }
        println!();
    }
}
