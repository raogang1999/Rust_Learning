use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{SmtpClient,Transport};
use lettre_email::{EmailBuilder,Mailbox};

fn main() {

    let email = EmailBuilder::new()
        .from("111@qq.com".to_string())
        .to(Mailbox::new("11@163.com".to_string()))
        .subject("Test")
        .body("This is a test email")
        .build()
        .unwrap();
    let creds = Credentials::new("11@qq.com".to_string(),"111".to_string());

    let mut mailer = SmtpClient::new_simple("smtp.qq.com")
        .unwrap()
        .credentials(creds)
        .authentication_mechanism(Mechanism::Login)
        .transport();
    let result = mailer.send(email.into());
    if result.is_ok(){
        println!("发送成功")
    }else {
        print!("发送失败");
        println!("{:?}", result);
    }
}
