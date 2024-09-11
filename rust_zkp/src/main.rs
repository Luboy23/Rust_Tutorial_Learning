use rand::{distributions::Alphanumeric, Rng};

// 定义用户结构体
struct User {
    username: String,
    password: String,
}

// 定义服务器结构体
struct Server {
    salt: String,
}

impl Server {
    // 认证函数，比较用户响应是否与服务器期望响应匹配
    fn authenticate(&self, user: &User, response: &str) -> bool {
        let challenge: &String = &self.salt;
        let expected_response: String = hash(user.password.clone(), challenge);

        // 比较传入的响应和期望响应
        response == expected_response
    }
}

// 简单的哈希函数，模拟将密码和 salt 拼接起来
fn hash(input: String, salt: &str) -> String {
    format!("Hash({} + {})", input, salt)
}

fn main() {
    // 创建用户
    let user: User = User {
        username: "lllu_23".to_string(),
        password: "Password123".to_string(),
    };

    // 生成服务器的随机 salt 
    let server: Server = Server {
        salt: rand::thread_rng()
            .sample_iter(&Alphanumeric) // 修正错误的 `sample.iter`
            .take(16) // 随机生成 16 个字符的 slat
            .map(char::from)
            .collect(),
    };

    // 使用用户密码和服务器 sa l t生成响应
    let response: String = hash(user.password.clone(), &server.salt);

    // 进行身份验证
    if server.authenticate(&user, &response) {
        println!("Authentication successful for user: {}", user.username);
    } else {
        println!("Authentication failed for user: {}", user.username);
    }
}