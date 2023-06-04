#[cfg(test)]
mod tests {

    use sqlx::mysql::MySqlPoolOptions;
    use sqlx::MySqlPool;

    #[sqlx::test]
    async fn create_connection(){
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect("mysql://root:password@127.0.0.1:3306/emails")
            .await;
        match pool {
            Ok(_) => assert!(true),
            Err(err) => panic!("ERROR CONNECTING TO DATABASE: {}", err),
        }
    }

    #[sqlx::test]
    async fn add_email(pool: MySqlPool){
        match sqlx::query!(r#"INSERT INTO mailing_list (email) VALUES (?)"#, format!("example@test.com"))
            .execute(&pool)
            .await {
                Ok(_) => assert!(true),
                Err(err) => panic!("ERROR ADDING EMAIL: {}", err),
            }
    }

    #[sqlx::test]
    async fn remove_email(pool: MySqlPool){
        sqlx::query!(r#"INSERT INTO mailing_list (email) VALUES (?)"#, format!("example@test.com"))
            .execute(&pool)
            .await
            .expect("ERROR ADDING TEST EMAIL");

        match sqlx::query!(r#"DELETE FROM mailing_list WHERE email = (?)"#, format!("example@test.com"))
            .execute(&pool)
            .await {
                Ok(_) => assert!(true),
                Err(err) => panic!("ERROR ADDING EMAIL: {}", err),
            }
    }

    use lettre::transport::smtp::authentication::Credentials; 
    use lettre::{SmtpTransport, Transport};
    use lettre::message::{header::ContentType, Message};

    #[tokio::test]
    async fn new_job() {
        let smtp_username: String = String::from("username");
        let smtp_password: String = String::from("password");
        let sender: String = String::from("sender");
        let relay: String = String::from("relay");

        let creds = Credentials::new(smtp_username, smtp_password);
        let mailer = SmtpTransport::relay(&relay) 
            .expect("Relay Error") 
            .credentials(creds) 
            .build(); 

        let email = Message::builder() 
            .from(sender.clone().parse().unwrap()) 
            .to("test@test.com".parse().unwrap()) 
            .subject("Newsletter") 
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("Newsletter test"))
            .unwrap(); 

        match mailer.send(&email) { 
              Ok(_) => assert!(true), 
              Err(e) => panic!("Could not send email: {:?}", e), 
            }
    }
}
