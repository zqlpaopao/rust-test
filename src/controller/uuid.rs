use uuid::Uuid;

pub fn test_uuid() {
    let id = Uuid::new_v4();
    println!("uuid v4: {}", id);
    //uuid v4: d57a3a40-cc31-4532-8df4-cf5c20cd3f32

    // SHA-1 哈希生成uuid v5 特性
    // DNS域的命名空间UUID
    const NAMESPACE_DNS: Uuid = Uuid::from_u128(0x6ba7b810_9dad_11d1_80b4_00c04fd430c8);

    // 要hash的数据
    let name = "example.com";

    // 创建版本5的UUID
    let id = Uuid::new_v5(&NAMESPACE_DNS, name.as_bytes());
    println!("v5 {}", id);
    //v5 cfbff0d1-9375-5685-968c-48ce8b15ae17

    // v6 基于时间戳和具有不同布局的单调计数器
    // v6：基于时间戳和具有不同布局的单调计数器
    // v7：基于Unix时间戳
    // v8：基于用户定义数据

    //可以使用hyphenated()方法将UUID格式化为带连字符的十六进制字符串
    let id = Uuid::new_v4();
    // 将UUID格式化为带连字符的字符串
    let output = id.hyphenated().to_string();
    println!("{}", output);
    //d27d0715-90a0-4cc3-ae5b-223da8e66bb6

    // 随机UUID urn()方法将UUID格式化为统一资源名称(URN)：
    let id = Uuid::new_v4();
    // 将UUID格式化为URN
    let output = id.urn().to_string();
    println!("{}", output);
    //urn:uuid:d51d5d61-d44e-4310-8b69-c20177ede75d

    // 随机UUID 可以使用simple()方法将UUID格式化为不带连字符(-)的简单字符串：
    let id = Uuid::new_v4();
    // 将UUID格式化为一个简单的字符串
    let output = id.simple().to_string();
    println!("{}", output);
    //0c874e7032a2491c9b943d743898992d
}
