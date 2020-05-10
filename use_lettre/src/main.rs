use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::{EmailBuilder, Mailbox};

fn main() {
    let email = EmailBuilder::new()
        .from(Mailbox::new("发送者的邮箱地址".to_string())) 
        //.from(Mailbox::new("xiaoming@163.com".to_string())) //发送者：xiaoming@163.com
        .to(Mailbox::new("接收者邮箱地址".to_string()))
        //.to(Mailbox::new("xiaohong@126.com".to_string())) //接收者：xiaohong@126.com
        .subject("Test") //邮件标题
        .body("This is a test email!") //邮件内容
        .build()
        .unwrap();

    //for example: xiaoming@163.com, password: 123456
    //let creds = Credentials::new("xiaoming".to_string(), "123456".to_string());
    let creds = Credentials::new("你的邮箱用户名".to_string(), "你的邮箱密码".to_string());

    //如163的邮箱就是smtp.163.com, 126的邮箱就是smtp.126.com
    let mut mailer = SmtpClient::new_simple("邮箱服务器地址") 
        .unwrap()
        .credentials(creds)
        .transport();

    let result = mailer.send(email.into());

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }

    assert!(result.is_ok());
}
