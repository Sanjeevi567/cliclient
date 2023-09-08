//Foreign crates
use colored::Colorize;
use inquire::{
    ui::{Attributes, RenderConfig, StyleSheet, Styled},
    Confirm, Password, Select, Text,
};

use std::fs::OpenOptions;
use std::io::Write;

//Interneal APIs

use aws_apis::{
    load_credential_from_env, CredentInitialize, MemDbOps, RdsOps, S3Ops, SesOps, SimpleMail,
    Simple_, TemplateMail, Template_,
};

#[tokio::main]
async fn main() {
    inquire::set_global_render_config(global_render_config());

    let operations: Vec<&str> = vec![
        "Verify the Credential\n",
        "Print Crdentials Information\n",
        "AWS Simple Email Service(SES) Operations\n",
        "S3 Bucket Operations\n",
        "Relational Database Service(RDS) Operations\n",
        "MemoryDb Operations\n",
        "Quit the application\n",
    ];
    //Intial dummy credentials
    let mut credential = CredentInitialize::default();
    //Using same credentials for the different services.
    let mut ses_ops: SesOps = SesOps::build(credential.build());
    let mut s3_ops: S3Ops = S3Ops::build(credential.build());
    let mut rds_ops: RdsOps = RdsOps::build(credential.build());
    let mut memdb_ops: MemDbOps = MemDbOps::build(credential.build());

    'main: loop {
        let choice = Select::new("Operations to perform\n", operations.clone())
            .with_help_message("Don't enclose data in quotation marks or add spaces around it in any operations,\nexcept when working with template data.")
            .prompt()
            .unwrap();

        match choice {
            "Verify the Credential\n" => {
                let choices = Confirm::new("Please load the credentials or enter them manually\n")
                          .with_placeholder("Use 'Yes' to load from the environment, and 'No' to provide the information manually\n")
                          .with_help_message("message")
                          .prompt()
                          .unwrap();

                match choices {
                    true => {
                        let (credentials, region) = load_credential_from_env().await;
                        credential.update(
                            credentials.access_key_id(),
                            credentials.secret_access_key(),
                            Some(&region),
                        );
                        let config = credential.build();
                        ses_ops = SesOps::build(config.clone());
                        s3_ops = S3Ops::build(config.clone());
                        rds_ops = RdsOps::build(config.clone());
                        memdb_ops = MemDbOps::build(config);

                        println!("{}\n","Please verify the credentials by printing the credential information before proceeding with any operations".red().bold());
                    }

                    false => {
                        let access_key = Text::new("Please provide the access key\n")
                            .with_placeholder("The access key is provided by AWS\n")
                            .with_formatter(&|str| format!(".....{str}....."))
                            .prompt()
                            .unwrap();
                        let secret_key_confirm = Confirm::new("Visibility of the secret key while typing\n")
                            .with_placeholder("Select 'Yes' to display the key while you are typing, or 'No' to hide the characters as you type\n")
                            .prompt()
                            .unwrap();
                        let secret_key = match secret_key_confirm {
                            true => Text::new("")
                                .with_placeholder("Utilize AWS services to acquire the key\n")
                                .prompt()
                                .unwrap(),
                            false => Password::new("Enter the Secret Access key\n")
                                .with_help_message("Utilize AWS services to acquire the key.n")
                                .without_confirmation()
                                .prompt()
                                .unwrap(),
                        };

                        let region = Text::new("Please enter the name of the region\n")
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .with_placeholder("The chosen region name should allow for operations in the command line.\n")
                            .prompt()
                            .unwrap();
                        match (
                            access_key.is_empty(),
                            secret_key.is_empty(),
                            region.is_empty(),
                        ) {
                            (false, false, false) => {
                                credential.update(&access_key, &secret_key, Some(&region));
                                let config = credential.build();
                                ses_ops = SesOps::build(config.clone());
                                s3_ops = S3Ops::build(config);
                                println!("{}\n","Please verify the credentials by printing the credential information before proceeding with any operations".red().bold());
                            }
                            _ => println!(
                                "{}\n",
                                "Credential informations can't be empty".red().bold()
                            ),
                        }
                    }
                }
            }
            "Print Crdentials Information\n" => {
                let confirm =
                    Confirm::new("Are you sure you want to print credential information?\n")
                        .with_formatter(&|str| format!(".....{str}.....\n"))
                        .with_placeholder("This is solely for verification purposes\n")
                        .with_default(false)
                        .prompt()
                        .unwrap();

                match confirm {
                    true => {
                        println!("Here is your credential informations\n");
                        println!("{:#?}", credential.get_credentials());
                    }
                    false => {
                        println!("{}\n", "Sure...".green().bold())
                    }
                }
            }
            "AWS Simple Email Service(SES) Operations\n" => {
                let ses_operations = vec![
    "Create a Contact List Name\n",
    "Add an email to the list\n",
    "Email Verification\n",
    "Configure from_address,list_name and template_name for\n one-time usage or customize them to better suit your specific use case\n",
    "Print the emails from the provided list\n",
    "Send a Simple Email to a Specific Recipient\n",
    "Send a Templated Email to a Specified Email Address\n",
    "Send a simple email with the same body and subject to all the email addresses in the list\n",
    "Send templated Emails\n",
    "Common Errors\n",
    "Go to main menu\n",
    ];
                loop {
                    let email_choice =
                        Select::new("Operations to perform\n", ses_operations.clone())
                            .with_help_message(
                                "Do not enclose it with quotation marks or add spaces",
                            )
                            .with_vim_mode(true)
                            .with_page_size(5)
                            .prompt()
                            .unwrap();

                    match email_choice {
            "Create a Contact List Name\n" => 
            
            {
                let lst_name = Text::new("Enter the list name to add to the AWS Simple Email Service\n")
                    .with_placeholder("The name should be unique")
                    .with_help_message("This is where the emails are stored")
                    .prompt()
                    .unwrap();
                let description = Text::new("Small Description about the list name\n")
                    .with_placeholder("Eg: A list named 'Zone Email Contacts' is used to add the emails\nof people in a specific area but can be skipped\n")
                    .prompt_skippable()
                    .unwrap()
                    .unwrap();
                match (lst_name.is_empty(), description.is_empty()) {
                    (false, false) => {
                        ses_ops.create_contact_list_name(&lst_name, Some(description)).await;
                    }
                    (false,true) =>{
                        ses_ops.create_contact_list_name(&lst_name, None).await;
                    },
                    _ => println!("{}\n","Contact Name Can't be empty..try again".red().bold()),
                }
            }

            "Add an email to the list\n" => {
                let get_contact_list_name = ses_ops.get_list_name();
                let get_contact_list_name = format!("Default contact list name: {}\n",get_contact_list_name);          
                let  email = Text::new("Enter the email\n")
                    .with_placeholder("Emails should be without quotation marks around them\n")
                    .with_formatter(&|str| format!(".....{str}.....\n"))
                    .prompt()
                    .unwrap();
                let list_name = Text::new("Enter the list name you want the email add in it\n")
                                     .with_placeholder(&get_contact_list_name)
                                     .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt_skippable()
                                    .unwrap()
                                    .unwrap();
                let to_verified = Confirm::new("Would you like to send the verification email as well?\n")
                                           .with_formatter(&|str| format!(".....{str}.....\n"))
                                           .with_placeholder("Selecting 'Yes' means you want to receive a verification, while choosing 'No' means your email will be added to the list without verification\n")
                                           .prompt()
                                           .unwrap();
                                           
                                
                match (list_name.is_empty(),email.is_empty(),to_verified) {
                    (false,false,false) => {
                        println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                        ses_ops.create_email_contact_without_verification(&email, Some(&list_name)).await;
                    },
                    (false,false,true) =>{
                        println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                        ses_ops.create_email_contact_with_verification(&email,Some(&list_name)).await;
                    }
                   (true,false,false) =>{
                      println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                       ses_ops.create_email_contact_without_verification(&email,None).await;
                   }
                   (true,false,true) =>{
                    println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                      ses_ops.create_email_contact_with_verification(&email,None).await;
                   }
                    _ => println!("{}\n","No email is received".red().bold()),
                }
            }

            "Email Verification\n" =>{
                let email_to_verify = Text::new("Enter the email to check the identity\n")
                                    .with_placeholder("Only verified email can receive email\n")
                                    .prompt()
                                    .unwrap();
                        match email_to_verify.is_empty(){
                            false => {
                                println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                               match ses_ops
                                    .is_email_verfied(&email_to_verify)
                                     .await{
                                      true =>{
                                        let email_to_verify = email_to_verify.green().bold();
                                        println!("The email address {email_to_verify} has been verified\n");
                                        println!(" You can use it to receive messages or as a 'from' address\n");
                                    },
                                      false => {
                                        let email_to_verify = email_to_verify.green().bold();
                                        println!("The email address {email_to_verify} is not verified\n");
                                        println!("Therefore, you can't use it to send emails ('from' address) or receive messages\n");
                                      }
                                }

                            }
                            true =>{
                               println!("{}\n","The email can't be empty".red().bold())
                            }

                        }

            }


            "Print the emails from the provided list\n" => {
              
                let get_contact_list_name=ses_ops.get_list_name();
                let get_contact_list_name = format!("Default contact list name: {}\n",get_contact_list_name);
                

                  let list_name = Text::new("Please enter the name of the list for which you'd like to print emails\n")
                                       .with_placeholder(&get_contact_list_name)
                                       .with_formatter(&|str| format!(".....{str}....."))
                                       .prompt_skippable()
                                       .unwrap()
                                       .unwrap();
                 let print_emails = Confirm::new("You are tasked with printing all the emails in the list\n")
                                       .with_placeholder("Select 'Yes' to print emails or 'No' to save them to the current directory\n")
                                       .with_formatter(&|str| format!(".....{str}....."))
                                       .prompt_skippable()
                                       .unwrap()
                                       .unwrap();
                 match list_name.is_empty() {
                     false =>{
                           match print_emails{
                            true =>{
                                let upto = Text::new("How many emails would you like to print?\n")
                                                     .with_placeholder("Values should be non-zero; if no value is provided, it defaults to zero")
                                                     .with_formatter(&|str| format!(".....{str}....."))
                                                     .prompt_skippable()
                                                     .unwrap()
                                                     .unwrap();
                                    match upto.is_empty(){
                                        false =>{
                                            println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                                            let parse_to_digit = upto.parse::<usize>().unwrap();
                                            ses_ops.printing_email_addresses_from_provided_list(Some(&list_name), true, Some(parse_to_digit)).await;

                                        }
                                        true =>{
                                            println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                                            ses_ops.printing_email_addresses_from_provided_list(Some(&list_name), true, None).await;

                                        }

                                    }

                            },
                            false =>{
                            ses_ops.printing_email_addresses_from_provided_list(Some(&list_name), false, None).await;
  
                            }
                           }
                     }
                     true => {
                        match print_emails{
                            true =>{
                                let upto = Text::new("How many emails would you like to print?\n")
                                                     .with_placeholder("Values should be non-zero; if no value is provided, it defaults to zero")
                                                    .with_formatter(&|str| format!(".....{str}....."))
                                                     .prompt_skippable()
                                                     .unwrap()
                                                     .unwrap();
                                    match upto.is_empty(){
                                        false =>{
                                            println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                                            let parse_to_digit = upto.parse::<usize>().unwrap();
                                            ses_ops.printing_email_addresses_from_provided_list(None, true, Some(parse_to_digit)).await;

                                        }
                                        true =>{
                                            println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());  
                                            ses_ops.printing_email_addresses_from_provided_list(None, true, None).await;

                                        }

                                    }

                            },
                            false =>{
                                println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                                ses_ops.printing_email_addresses_from_provided_list(None, false, None).await;  
                            }
                         

                     }
                 }                   
                    
 }},

 "Configure from_address,list_name and template_name for\n one-time usage or customize them to better suit your specific use case\n" =>{

    let get_from_address = ses_ops.get_from_address();
    let get_template_name = ses_ops.get_template_name();
    let get_list_name= ses_ops.get_list_name();

    let default_from_address = format!("Default from_address is: {}",get_from_address);
    let default_list_name = format!("Default list name is: {}",get_list_name);
    let default_template_name = format!("Default template name is: {}",get_template_name);

                    let list_name = Text::new("Provide a name for the default list\n")
                            .with_placeholder("The list name can be omitted if it has been set previously,\nunless you intend to change it to a different default list name")
                            .with_formatter(&|str| format!(".....{str}....."))
                            .with_help_message(&default_list_name)
                            .prompt_skippable()
                            .unwrap()
                            .unwrap();
                     let template_name = Text::new("Provide a name for the default template name\n")
                           .with_placeholder("The template name can be omitted if it has been set previously,\nunless you intend to change it to a different default template name")
                           .with_formatter(&|str| format!(".....{str}....."))
                           .with_help_message(&default_template_name)
                           .prompt_skippable()
                           .unwrap()
                           .unwrap();
                      let from_address = Text::new("Provide a email for the default from address\n")
                          .with_formatter(&|str| format!(".....{str}....."))
                          .with_placeholder("The from_address name can be omitted if it has been set previously, unless you intend to change it to a different default from_address name")
                          .with_help_message(&default_from_address)
                          .prompt_skippable()
                          .unwrap()
                          .unwrap();
                    match (list_name.is_empty(),template_name.is_empty(),from_address.is_empty()){

                //If both the template name and from address are skipped    
                        (false,true,true) =>{
                                 ses_ops.set_list_name(&list_name);
                                 println!("{} {}\n","Default list name is:".bright_blue(),list_name);
                                 println!("{}\n","make sure look at the placeholder to know the dafaults".bright_blue());
                                 
                        },
                //If both the list name and from address are skipped
                        (true,false,true) =>{
                            ses_ops.set_template_name(&template_name);
                            println!("{} {}\n","Default Template Name is:".bright_blue(),ses_ops.get_template_name());
                            println!("{}\n","Be sure to check the placeholder for default values, allowing you to skip using the default value".bright_blue());
                        },
                 //If both the template name and list name are skipped       
                        (true,true,false) => {
                            ses_ops.set_from_address(&from_address);
                            println!("{} {}\n","Default from_address is:".bright_blue(),ses_ops.get_from_address());
                            println!("{}\n","Be sure to check the placeholder for default values, allowing you to skip using the default value".bright_blue());
                        },
                //If the from address is skipped       
                        (false,false,true) =>{
                            ses_ops.set_list_name(&list_name);
                            ses_ops.set_template_name(&template_name);
                            println!("{} {}\n{} {}\n","Default List name is:".bright_blue(),ses_ops.get_list_name(),
                            "Default Template Name is:".bright_blue(),ses_ops.get_template_name());
                            println!("{}\n","Be sure to check the placeholder for default values, allowing you to skip using the default value".bright_blue());
                        },
                  //If the template name is skipped      
                        (false,true,false) =>{
                            ses_ops.set_list_name(&list_name);
                            ses_ops.set_from_address(&from_address);
                            println!("{} {}\n{} {}\n","Default List Name is:".bright_blue(),ses_ops.get_list_name(),
                            "Default from_address is:".bright_blue(),ses_ops.get_from_address());
                            println!("{}\n","Be sure to check the placeholder for default values, allowing you to skip using the default value".bright_blue());
                        },
                   //If the list name is skipped.    
                        (true,false,false) =>{
                                ses_ops.set_template_name(&template_name);
                                ses_ops.set_from_address(&from_address);
                                println!("{} {}\n{} {}\n","Default Template Name is:".bright_blue(),ses_ops.get_template_name(),
                                "Default from_address is:".bright_blue(),
                                ses_ops.get_from_address());
                                println!("{}\n","Be sure to check the placeholder for default values, allowing you to skip using the default value".bright_blue());
                        },
                // If a list name, template, and from address are provided...    
                        (false,false,false) =>{
                            ses_ops.set_list_name(&list_name);
                            ses_ops.set_template_name(&template_name);
                            ses_ops.set_from_address(&from_address);
                            println!("{} {}\n{} {}\n{} {}\n","Default List Name is:".bright_blue(),ses_ops.get_list_name(),"Default Template Name is:".bright_blue(),ses_ops.get_template_name(),
                            "Default from_address is:".bright_blue(),ses_ops.get_from_address());
                            println!("{}\n","Be sure to check the placeholder for default values, allowing you to skip using the default value".bright_blue());
                        },
                //All three can't be empty since there is no reason to choose this option if none of them is modified 
                        _ =>{

                        println!("{}\n","All three can't be empty since there is no reason to choose this option if none of them is modified".red().bold());
                        }
                    
                    }

                        }

            "Send a Simple Email to a Specific Recipient\n" => {

                let email = Text::new("Enter the email..\n")
                                   .with_formatter(&|str| format!(".....{str}....."))
                                   .prompt()
                                   .unwrap();
                println!("{email}");

                let subject = Text::new("Enter the subject of Email\n")
                                 .with_formatter(&|str| format!(".....{str}....."))
                                 .prompt().unwrap();

                let body = Text::new("Enter the message body to display to the recipient\n")
                                 .with_formatter(&|str| format!(".....{str}....."))
                                .prompt()
                                .unwrap();
                 
                let defaul_from_address = ses_ops.get_from_address();
        
                let default_from_address =format!("Your 'from_address' needs to be verified, which is typically your email\nand the default 'from_address' is {}",defaul_from_address);

                let from_address = Text::new("Enter the 'from' address or press enter to use default from_address if any\n")
                    .with_placeholder(&default_from_address)
                    .with_formatter(&|str| format!(".....{str}....."))
                    .prompt_skippable()
                    .unwrap()
                    .unwrap();
                let simple_email = SimpleMail::builder(
                    &body,
                    &subject
                )
                .build();

                match (email.is_empty(), subject.is_empty(), body.is_empty(),from_address.is_empty()) {
                    (false, false, false,false) => {
                             ses_ops
                            .send_mono_email(&email, Simple_(simple_email),Some(&from_address))
                            .await
                            .send()
                            .await
                            .map(|_|{
                                let colored_email = email.green().bold();
                                println!("An Email is send succesfully to: {}",colored_email)
                              })
                            .unwrap();
                          
                    }
                    (false,false,false,true) => {
                          
                                 ses_ops
                                .send_mono_email(&email, Simple_(simple_email),None)
                                .await
                                .send()
                                .await
                                .map(|_|{
                                    let colored_email = email.green().bold();
                                    println!("An Email is send succesfully to: {}",colored_email)
                                   })
                                .unwrap();
                          
                    }
                    _ => {
                        println!("{}\n","Email,subject or body can't be empty".red().bold());
                    }
                }
            }
            "Send a Templated Email to a Specified Email Address\n" => {

                let get_from_address = ses_ops.get_from_address();
                let get_template_name = ses_ops.get_template_name();
                let default_template_name = format!("Default template name is: {}",get_template_name);
                let default_from_address = format!("Default from_address is: {}",get_from_address);
                let email = Text::new("Enter the email you wish to send\n")
                    .with_placeholder("The email must be verified")
                    .with_formatter(&|str| format!(".....{str}....."))
                    .prompt()
                    .unwrap();
    
        let template_name = Text::new(
                "Please enter the template name you want to use for the email\n",)
            .with_placeholder(&default_template_name)
            .with_help_message(
                "The template name must exist, and the variables should be specified as key-value pairs according to the template\n",
            )
            .prompt_skippable()
            .unwrap()
            .unwrap();
            
            let from_address =Text::new("Enter the from address\n")
                .with_placeholder(&default_from_address)
                .with_formatter(&|str| format!(".....{str}.....\n"))
                .prompt_skippable()
                .unwrap()
                .unwrap();
            let template_data = Text::new("Enter the template data\n")
                      .with_formatter(&|str| format!(".....{str}.....\n"))
                      .prompt()
                      .unwrap();
                 
                match (email.is_empty(), template_name.is_empty(),from_address.is_empty()) {
                    //If both email and template is specified we can use those
                    (false, false,false) => {
                        let email_content=TemplateMail::builder(&template_name, &template_data)
                        .build();
                         ses_ops
                        .send_mono_email(&email, Template_(email_content),Some(&from_address))
                        .await
                        .send()
                        .await
                        .map(|_|{
                            let colored_email = email.green().bold();
                            println!("The template email is send to: {}\n",colored_email)
                    })
                        .unwrap();
                    }
                    //If template name is skipped then default template name is used
                    //which might results in error if no template name is None or not exist
                    (false,false,true) => {
                        let email_content=TemplateMail::builder(&template_name, &template_data)
                        .build();
                            ses_ops
                            .send_mono_email(&email, Template_(email_content),None)
                            .await
                            .send()
                            .await
                            .map(|_|{
                                let colored_email = email.green().bold();
                                println!("The template email is send to: {}\n",colored_email)
                                 })
                            .unwrap();
                    }
                
                    (false,true,true) =>{
                        println!("Template data is:\n{}",template_data);
                        let email_content=TemplateMail::builder(get_template_name, &template_data)
                        .build();
                            ses_ops
                            .send_mono_email(&email, Template_(email_content),None)
                            .await
                            .send()
                            .await
                            .map(|_|{
                                let colored_email = email.green().bold();
                                println!("The template email is send to: {}\n",colored_email)})
                            .unwrap();
                    }
                    (false,true,false) =>{
                        let email_content=TemplateMail::builder(get_template_name, &template_data)
                        .build();
                            ses_ops
                            .send_mono_email(&email, Template_(email_content),Some(&from_address))
                            .await
                            .send()
                            .await
                            .map(|_|{
                                let colored_email = email.green().bold();
                                println!("The template email is send to: {}\n",colored_email)})
                            .unwrap();
                    }
                    _ => {
                        println!("{}\n","Please ensure the email field is not empty, and try again".red().bold());                      
                    }
                }
            }
            "Send a simple email with the same body and subject to all the email addresses in the list\n" => {
                 
                let get_from_address = ses_ops.get_from_address();
                let get_list_name=ses_ops.get_list_name();

                let default_from_address = format!("Default from_address is: {}\n",get_from_address);
                let default_list_name = format!("Default list name is: {}\n",get_list_name);   

                let list_name = Text::new("Enter the list name..\n")
                    .with_placeholder(&default_list_name)
                    .with_formatter(&|str| format!(".....{str}.....\n"))
                    .prompt_skippable()
                    .unwrap()
                    .unwrap_or(ses_ops.get_list_name().into());
                let body = Text::new("Body details\n")
                    .with_placeholder("The body data is the same for all emails..\n")
                    .with_formatter(&|str| format!(".....{str}.....\n"))
                    .prompt()
                    .unwrap();
                let subject = Text::new("Enter the subject name\n")
                    .with_placeholder("The subject is the same for all emails\n")
                    .with_formatter(&|str| format!(".....{str}.....\n"))
                    .prompt()
                    .unwrap();
                let from_address = Text::new("Enter the from address\n")
                             .with_placeholder(&default_from_address)
                             .with_formatter(&|str| format!(".....{str}.....\n"))
                             .prompt_skippable()
                             .unwrap()
                             .unwrap();
                let simple_data = SimpleMail::builder(&body, &subject);
                match (subject.is_empty(),body.is_empty()){
                    (false,false)=>{
                          match (list_name.is_empty(),from_address.is_empty()){
                            (false,false) => {
                                 ses_ops
                                .send_multi_email_with_simple(simple_data, Some(&from_address), Some(&list_name))
                                .await; 
                            }
                            (true,true) =>{
                                ses_ops
                                .send_multi_email_with_simple(simple_data, None, None)
                                .await;
                            }
                            (false,true) =>{
                                ses_ops
                                .send_multi_email_with_simple(simple_data, None, Some(&list_name))
                                .await;
                            }
                            (true,false) =>{
                               ses_ops
                                .send_multi_email_with_simple(simple_data, Some(&from_address), None)
                                .await;
                            }
                          }
                    }
                    _ => println!("{}\n","Both body and subject can't be empty".red().bold()),
                }
            }

            "Send templated Emails\n" => {
                let get_from_address = ses_ops.get_from_address();
                let get_template_name = ses_ops.get_template_name();
                let get_list_name=ses_ops.get_list_name();

                println!("{}\n","This information is for debugging".green().bold());

                let template_name = format!("Template name is: {}\n",get_template_name);
                let list_name = format!("List name is: {}\n",get_list_name);
                let from_address = format!("from address is: {}\n",get_from_address);


                println!("{}\n{}\n{}",template_name,list_name,from_address);
                  ses_ops
                  .send_emails()
                  .await;
            }
         "Common Errors\n" => {
            let possible_errors = include_str!("./possible_errors.txt").blue().italic().bold();
             println!("{}\n",possible_errors);

            let decision = Confirm::new("Go to main or exist")
               .with_placeholder("Yes to main menu , No to Quit")
              .prompt()
              .unwrap();
            if decision{
                continue 'main;
            }
            else{
                break 'main;
            }

         }
         "Go to main menu\n" => continue 'main,

            _ => {}
        }
                }
            }

            "S3 Bucket Operations\n" => {
                let s3_operations = vec![
                    "Create Bucket\n",
                    "Put object in a Bucket\n",
                    "List objects from a Bucket\n",
                    "Download object from bucket\n",
                    "Retrieve a presigned URL for an object\n",
                    "Get Bucket Lists\n",
                    "Delete object from a bucket\n",
                    "Delete Bucket\n",
                    "Go to main\n",
                ];

                's3_ops: loop {
                    let s3_choices = Select::new("Operations in S3 service", s3_operations.clone())
                        .with_page_size(5)
                        .prompt()
                        .unwrap();
                    match s3_choices {
                        "Create Bucket\n" => {
                            let get_bucket_lists = s3_ops.get_buckets().await;
                            let existing_buckets = format!(
                                "These buckets are already in your account: {:#?}",
                                get_bucket_lists
                            );
                            let bucket_name = Text::new("Please input the name of the bucket\n")
                                .with_placeholder(
                                    &existing_buckets
                                )
                                .with_help_message("The name must begin with a lowercase letter and should be unique\nAn AWS bucket is a type of object storage designed for storing objects")
                                .with_formatter(&|str| format!("The Bucket name is: {str} and assumes that region name is provided"))
                                .prompt()
                                .unwrap();
                            match bucket_name.is_empty() {
                                false => {
                                    s3_ops.create_bucket(&bucket_name).await;
                                }
                                true => println!(
                                    "{}\n",
                                    "The bucket name can't be empty; please try again"
                                        .red()
                                        .bold()
                                ),
                            }
                        }
                        "Get Bucket Lists\n" => {
                            println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                            let bucket_lists = s3_ops.get_buckets().await;
                            for bucket in bucket_lists {
                                println!("    {}\n", bucket.green().bold());
                            }
                        }

                        "List objects from a Bucket\n" => {
                            let get_bucket_name = s3_ops.get_buckets().await;
                            let bucket_names =
                                format!("Available buckets are: {:#?}\n", get_bucket_name);

                            let bucket_name = Text::new("Please input the name of the bucket")
                                .with_placeholder(&bucket_names)
                                .prompt()
                                .unwrap();

                            match bucket_name.is_empty() {
                                false => {
                                    println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                                    let object_names =
                                        s3_ops.retrieve_keys_in_a_bucket(&bucket_name).await;
                                    for object in object_names {
                                        let colored_key_name = object.green().bold();
                                        println!("    {}\n", colored_key_name);
                                    }
                                }
                                true => {
                                    println!(
                                        "{}\n",
                                        "The bucket name can't be empty; please try again"
                                            .red()
                                            .bold()
                                    )
                                }
                            }
                        }
                        "Delete object from a bucket\n" => {
                            let get_bucket_lists = s3_ops.get_buckets().await;
                            let existing_buckets = format!(
                                "These buckets are already in your account: {:#?}",
                                get_bucket_lists
                            );
                            let bucket_name = Text::new("Please input the name of the bucket\n")
                                .with_placeholder(&existing_buckets)
                                .with_formatter(&|str| format!(".....{str}....."))
                                .prompt()
                                .unwrap();

                            match bucket_name.is_empty() {
                                false => {
                                    let object_names =
                                        s3_ops.retrieve_keys_in_a_bucket(&bucket_name).await;
                                    let available_object_names = format!(
                                        "The object names are in the {bucket_name} bucket: {:#?}\n",
                                        object_names
                                    );
                                    let object_name =
                                        Text::new("Enter the object/key name to delete\n")
                                            .with_placeholder(&available_object_names)
                                            .prompt()
                                            .unwrap();
                                    s3_ops
                                        .delete_content_in_a_bucket(&bucket_name, &object_name)
                                        .await;
                                }
                                true => {
                                    println!(
                                        "{}\n",
                                        "The bucket name can't be empty; please try again"
                                            .red()
                                            .bold()
                                    )
                                }
                            }
                        }

                        "Delete Bucket\n" => {
                            let get_bucket_name = s3_ops.get_buckets().await;
                            let bucket_names = format!(
                                "Below, you'll find the buckets in your account: {:?}\n",
                                get_bucket_name
                            );
                            let bucket_name = Text::new("Enter the bucket name to delete")
                                .with_placeholder(&bucket_names)
                                .with_help_message("These are the buckets within your AWS services")
                                .prompt()
                                .unwrap();
                            match bucket_name.is_empty() {
                                false => {
                                    s3_ops.delete_bucket(&bucket_name).await;
                                }
                                true => {
                                    println!(
                                        "{}\n",
                                        "The bucket name can't be empty; please try again"
                                            .red()
                                            .bold()
                                    )
                                }
                            }
                        }
                        "Put object in a Bucket\n" => {
                            let object = Text::new("Enter the object/data path\n")
                                .with_placeholder(
                                    "You can copy the path and ctrl+shift+v to paste it here",
                                )
                                .prompt()
                                .unwrap();

                            let get_bucket_name = s3_ops.get_buckets().await;
                            let available_bucket_name = format!(
                                "Available bucket names in your account: {:?}",
                                get_bucket_name
                            );
                            let bucket_name = Text::new("Enter the bucket name\n")
                                .with_placeholder(&available_bucket_name)
                                .with_help_message("This is where we put the actual data")
                                .prompt()
                                .unwrap();

                            let key = Text::new("Enter the key or the identifier\n")
                                .with_placeholder("This is what used to retreive the content later")
                                .prompt()
                                .unwrap();

                            match (object.is_empty(), bucket_name.is_empty(), key.is_empty()) {
                                (false, false, false) => {
                                    s3_ops
                                        .upload_content_to_a_bucket(&bucket_name, &object, &key)
                                        .await;
                                }

                                _ => {
                                    println!("{}\n", "Data ,the key/object name or the bucket name can't be empty".red().bold())
                                }
                            }
                        }

                        "Download object from bucket\n" => {
                            let get_buckets = s3_ops.get_buckets().await;
                            let available_buckets =
                                format!("Available buckets in your account: {:#?}\n", get_buckets);

                            let bucket_name = Text::new("Input the bucket name\n")
                                .with_placeholder(&available_buckets)
                                .prompt()
                                .unwrap();
                            match bucket_name.is_empty() {
                                false => {
                                    let get_objects =
                                        s3_ops.retrieve_keys_in_a_bucket(&bucket_name).await;
                                    let available_objects = format!(
                                        "Available keys in {bucket_name}\n{:#?}\n",
                                        get_objects
                                    );
                                    let object = Text::new("Input the object/key to download\n")
                                        .with_placeholder(&available_objects)
                                        .prompt()
                                        .unwrap();
                                    match object.is_empty() {
                                        false => {
                                            s3_ops
                                                .download_content_from_bcuket(&bucket_name, &object)
                                                .await;
                                        }
                                        true => {
                                            println!(
                                                "{}\n",
                                                "key/object name can't be empty".red().bold()
                                            )
                                        }
                                    }
                                }
                                true => {
                                    println!("{}\n", "Bucket name can't be empty".red().bold())
                                }
                            }
                        }

                        "Retrieve a presigned URL for an object\n" => {
                            let get_bucket_name = s3_ops.get_buckets().await;
                            let available_bucket_name = format!(
                                "Available buckets in your account: {:?}",
                                get_bucket_name
                            );
                            let bucket_name = Text::new("Enter the bucket name\n")
                                .with_placeholder(&available_bucket_name)
                                .with_help_message("This is where we put the actual data")
                                .prompt()
                                .unwrap();

                            match bucket_name.is_empty() {
                                false => {
                                    let get_objects =
                                        s3_ops.retrieve_keys_in_a_bucket(&bucket_name).await;
                                    let available_objects = format!(
                                        "Available keys in {bucket_name}\n{:#?}\n",
                                        get_objects
                                    );
                                    let object_name = Text::new("Enter the key or object for which you require a pre-signed URL\n")
                        .with_placeholder(&available_objects)
                        .prompt()
                        .unwrap();

                                    match object_name.is_empty() {
                                        false => {
                                            let choosing_hour = Text::new("Enter the expiration time for the url in hour\n")
                                    .with_placeholder("Integer values should always be non-negative and should not contain any characters\n")
                                    .prompt()
                                    .unwrap();
                                            match choosing_hour.is_empty() {
                                                false => {
                                                    let end_time =
                                                        choosing_hour.parse::<u64>().unwrap();
                                                    s3_ops
                                                        .get_presigned_url_for_an_object(
                                                            &bucket_name,
                                                            &object_name,
                                                            end_time,
                                                        )
                                                        .await;
                                                }
                                                true => println!(
                                                    "{}\n",
                                                    "the hour can't be empty".red().bold()
                                                ),
                                            }
                                        }
                                        true => println!(
                                            "{}\n",
                                            "key/object name can't be empty".red().bold()
                                        ),
                                    }
                                }
                                true => {
                                    println!("{}\n", "bucket name can't be empty".red().bold())
                                }
                            }
                        }
                        "Go to main\n" => break 's3_ops,
                        _ => {}
                    }
                }
            }

            "Relational Database Service(RDS) Operations\n" => {
                let rds_choices = vec![
                    "Create Db Instance\n",
                    "Configure Db and Cluster Instance Id\n",
                    "Describe Db Instance\n",
                    "Status of Db Instance\n",
                    "Retrieving Connection URL Information\n",
                    "Start Db Instance\n",
                    "Stop Db Instance\n",
                    "Modify Database Instance Settings\n",
                    "Delete Db Instance\n",
                    "Describe Db Cluster\n",
                    "Delete Db Cluster\n",
                    "Go to main menu\n",
                ];

                loop {
                    let choices =
                        Select::new("Select the operations to execute\n", rds_choices.clone())
                            .prompt()
                            .unwrap();
                    match choices {
                        "Create Db Instance\n" => {

                            let db_instance_identifier = Text::new("Enter the database instance identifier\n")
                               .with_placeholder("The DB instance identifier is case-insensitive, but is stored as all lowercase (as in \"mydbinstance\").\nConstraints: 1 to 60 alphanumeric characters or hyphens. First character must be a letter. Can't contain two consecutive hyphens. Can't end with a hyphen")
                               .prompt()
                               .unwrap();
                            let engine =
                                Text::new("Select the database engine for your database system\n")
                                    .with_placeholder(
                                        "Some possible values are: 'mariadb', 'mysql', 'postgres'",
                                    )
                                    .with_help_message(
                                        "look here to know more http://tinyurl.com/4h8fcwf6",
                                    )
                                    .prompt()
                                    .unwrap();
                            let db_name = Text::new("Select the db name for your database\n")
                                .with_placeholder(
                                    "Some possible values are: 'MySQL', 'MariaDB', 'PostgreSQL'",
                                )
                                .with_help_message(
                                    "look here to know more http://tinyurl.com/4mnhdpkm",
                                )
                                .prompt()
                                .unwrap();
                            let storage_type= Text::new("Select the storage type for your database")  
                     .with_placeholder("The storage type and the next database instance class should be a correct combination for successfully creating a database instance")   
                     .with_help_message("Click here http://tinyurl.com/4h8fcwf6 to learn more") 
                     .prompt()
                     .unwrap();
                            let db_instance_class =  Text::new("Select instance class for your database\n")  
            .with_placeholder("The instance class and the previous storage type should be a correct combination for successfully creating a database instance")   
            .with_help_message("Click here http://tinyurl.com/29am8kup to learn more") 
            .prompt()
            .unwrap();

                            let allocated_storage = Text::new("Specify the storage capacity for your database in gigabytes, using numerical digits\n")  
            .with_placeholder("The storage requirements depend on your specific use cases and the storage type you have previously selected")   
            .with_help_message("Click here http://tinyurl.com/4h8fcwf6 to learn more") 
            .prompt()
            .unwrap();

                            let username = Text::new("Select the username for your database\n")  
            .with_placeholder("The username and password options are required parameters for the database instance")  
            .prompt()
            .unwrap();
                            let password = Text::new("Select the password for your database\n")  
            .with_placeholder("Once you have created the database instance, you can obtain the database URL by selecting the 'Get Database URL' option")  
            .prompt()
            .unwrap();

                            match (
                                db_instance_identifier.is_empty(),
                                db_instance_class.is_empty(),
                                storage_type.is_empty(),
                                allocated_storage.is_empty(),
                                db_name.is_empty(),
                                engine.is_empty(),
                                username.is_empty(),
                                password.is_empty(),
                            ) {
                                (false, false, false, false, false, false, false, false) => {
                                    let storage = allocated_storage.parse::<i32>().unwrap();

                                    rds_ops
                                        .create_db_instance(
                                            &db_instance_identifier,
                                            &db_name,
                                            &db_instance_class,
                                            &engine,
                                            &username,
                                            &password,
                                            storage,
                                            &storage_type,
                                        )
                                        .await;

                            let mut file = OpenOptions::new().create(true).write(true)
                                                 .read(true).open("./create_db_instance_choices.txt").unwrap();
                            let choices = format!("Db Instance Identifier: {db_instance_identifier}\nDb Engine: {engine}\nDb Instance Class: {db_instance_class}\nAllocated Storage: {storage}\nStorage Type: {storage_type}\nMaster Username: {username}\nMaster Password: {password}\nDb Name: {db_name}");
                            
                           match file.write_all(choices.as_bytes()){
                               Ok(_) => {
                                let colored_msg ="The choices have been saved to the current directory for your reference\n".green().bold();
                                println!("{colored_msg}");
                               }
                               Err(_) => println!("Error while writting file to the current directory\n")
                            }
                            
                                }
                                _ => println!("{}\n", "Fields cannot be left empty.".red().bold()),
                            }
                        }

                        "Configure Db and Cluster Instance Id\n" => {
                            let db_instance_identifier = Text::new("Enter the database instance identifier\n")
     .with_placeholder("The database instance identifier is later used for all sorts of operations on the database, e.g., 'mydbinstance'")
     .prompt_skippable()
     .unwrap()
     .unwrap();
                            let db_cluster_identifier = Text::new("Enter the database cluster identifier, which is different from the database instance identifier\n")  
     .with_placeholder("This is the identifier you created when setting up a database cluster")
     .prompt_skippable()
     .unwrap()
     .unwrap();
                            match (
                                db_instance_identifier.is_empty(),
                                db_cluster_identifier.is_empty(),
                            ) {
                                (false, false) => {
                                    rds_ops.set_db_instance_id(&db_instance_identifier);
                                    rds_ops.set_db_cluster_id(&db_cluster_identifier);
                                    let colored_default_cluster_id =
                                        &db_cluster_identifier.green().bold();
                                    let colored_default_instance_id =
                                        &db_instance_identifier.green().bold();
                                    println!(
                                        "Default Instance Id: {colored_default_instance_id}\n"
                                    );
                                    println!("Default Cluster Id: {colored_default_cluster_id}\n");
                                    println!("{}\n","Be sure to check the placeholder for default values, allowing you to skip using the default value".bright_blue());
                                }
                                (true, false) => {
                                    rds_ops.set_db_cluster_id(&db_cluster_identifier);
                                    let colored_default_cluster_id =
                                        &db_cluster_identifier.green().bold();
                                    println!("Default Cluster Id: {colored_default_cluster_id}\n");
                                    println!("{}\n","Be sure to check the placeholder for default values, allowing you to skip using the default value".bright_blue());
                                }
                                (false, true) => {
                                    rds_ops.set_db_instance_id(&db_instance_identifier);
                                    let colored_default_instance_id =
                                        &db_instance_identifier.green().bold();
                                    println!(
                                        "Default Db Instance Id: {colored_default_instance_id}\n"
                                    );
                                    println!("{}\n","Be sure to check the placeholder for default values, allowing you to skip using the default value".bright_blue());
                                }
                                _ => println!("{}\n", "both fiedls can't be empty".red().bold()),
                            }
                        }

                        "Retrieving Connection URL Information\n" => {
                            let default_db_instance = format!("Default Db Instance Id: {}",rds_ops.get_db_instance_id());
                            let db_instance_identifier = Text::new("Enter the database instance identifier\n")  
                           .with_placeholder(&default_db_instance)
                            .prompt_skippable()
                            .unwrap()
                             .unwrap();

                            match db_instance_identifier.is_empty() {
         false => {
                         let postgres_choice = Confirm::new("Are you in need of a PostgreSQL connection URL?\n")
                             .with_placeholder("yes means ,proceed with the PostgreSQL option, No means you'll receive enough information about the database instance")
                             .prompt()
                             .unwrap();

                       match postgres_choice{
                            true => {

                             let password = Text::new("Enter the password\n")  
                              .with_placeholder("Please note that a password is necessary to generate the connection URL for the postgres database\n")
                              .prompt()
                              .unwrap();
                            let instance_info =rds_ops.describe_db_instance(Some(&db_instance_identifier)).await;
                            let username = instance_info.get_username();
                            let endpoint_with_port = instance_info.get_endpoint_with_port();
                            let db_name = instance_info.get_db_name();

                            match(username,endpoint_with_port,db_name,password.is_empty()){
                                (Some(username),Some(endpoint_with_port),Some(db_name),false) => {
                                    let database_url = format!("postgres://{username}:{password}@{endpoint_with_port}/{db_name}").green().bold();
                                    println!("The database url is: {}\n",database_url);
                                    rds_ops.status_of_db_instance(Some(&db_instance_identifier)).await;
                                },
                                _ => println!("Database url can't be generated\n")
                            }
                            }
                            false => {
                                let instance_info =rds_ops.describe_db_instance(Some(&db_instance_identifier)).await;
                                let username = instance_info.get_username();
                                let endpoint_with_port = instance_info.get_endpoint_with_port();
                                let db_name = instance_info.get_db_name();
                                
                                match(username,endpoint_with_port,db_name){
                                    (Some(username),Some(endpoint_with_port),Some(db_name)) => {
                                        let colored_username = username.blue().bold();
                                        let colored_endpoint_with_port = endpoint_with_port.blue().bold();
                                        let colored_db_name = db_name.blue().bold();
                                        println!("Username: {colored_username}\n");
                                        println!("Endpoint with port: {colored_endpoint_with_port}\n");
                                        println!("Db Name: {colored_db_name}\n");
                                        rds_ops.status_of_db_instance(Some(&db_instance_identifier)).await;
                                    },
                                    _ => println!("Database url can't be generated\n")
                                }
                            }
                         }
                                }

        true => {
                                    let postgres_choice = Confirm::new("Are you in need of a PostgreSQL connection URL?\n")
                                    .with_placeholder("yes means ,proceed with the PostgreSQL option, No means you'll receive enough information about the database instance")
                                    .prompt()
                                    .unwrap();
       
                              match postgres_choice{
                                   true => {
       
                                    let password = Text::new("Enter the password\n")  
                                     .with_placeholder("Please note that a password is necessary to generate the connection URL for the postgres database\n")
                                     .prompt()
                                     .unwrap();
                                   let instance_info =rds_ops.describe_db_instance(None).await;
                                   let username = instance_info.get_username();
                                   let endpoint_with_port = instance_info.get_endpoint_with_port();
                                   let db_name = instance_info.get_db_name();
       
                                   match(username,endpoint_with_port,db_name,password.is_empty()){
                                       (Some(username),Some(endpoint_with_port),Some(db_name),false) => {
                                           let database_url = format!("postgres://{username}:{password}@{endpoint_with_port}/{db_name}").green().bold();
                                           println!("The database url is: {}\n",database_url);
                                           rds_ops.status_of_db_instance(None).await;
                                       },
                                       _ => println!("Database url can't be generated\n")
                                   }
                                   }
                                   false => {
                                       let instance_info =rds_ops.describe_db_instance(Some(&db_instance_identifier)).await;
                                       let username = instance_info.get_username();
                                       let endpoint_with_port = instance_info.get_endpoint_with_port();
                                       let db_name = instance_info.get_db_name();
                                       
                                       match(username,endpoint_with_port,db_name){
                                           (Some(username),Some(endpoint_with_port),Some(db_name)) => {
                                               let colored_username = username.blue().bold();
                                               let colored_endpoint_with_port = endpoint_with_port.blue().bold();
                                               let colored_db_name = db_name.blue().bold();
                                               println!("Username: {colored_username}\n");
                                               println!("Endpoint with port: {colored_endpoint_with_port}\n");
                                               println!("Db Name: {colored_db_name}\n");
                                               rds_ops.status_of_db_instance(Some(&db_instance_identifier)).await;
                                           },
                                           _ => println!("Database url can't be generated\n")
                                       }
                                   }
                                }
                                }
                            }
                    
                        }

                        "Describe Db Instance\n" => {
                            let default_db_instance = format!("Default Db Instance Id: {}",rds_ops.get_db_instance_id());
                            let db_instance_identifier = Text::new("Enter the database instance identifier\n")  
                            .with_placeholder(&default_db_instance)
                            .prompt_skippable()
                            .unwrap()
                            .unwrap();

                            let instance_info = match db_instance_identifier.is_empty() {
                                false => {
                                    rds_ops
                                        .describe_db_instance(Some(&db_instance_identifier))
                                        .await
                                }

                                true => rds_ops.describe_db_instance(None).await,
                            };
                            let endpoint_with_port = instance_info.get_endpoint_with_port();
                            let zone = instance_info.get_availability_zone();
                            let class = instance_info.get_instance_class();
                            let db_name = instance_info.get_db_name();
                            let status = instance_info .get_instance_status();
                            println!("EndointWithPort: {:?}\nZone: {:?}\nInstance class: {:?}\nDb name: {:?}\nStatus of db instance: {:?}\n",
                            endpoint_with_port,zone,class,db_name,status);
                        }

                        "Start Db Instance\n" => {
                            let default_instance_id = format!(
                                "The default instance ID: {} and the current status db instance:\n",
                                rds_ops.get_db_instance_id(),
                            );
                            let db_instance_identifier =
                                Text::new("Enter the database instance identifier\n")
                                    .with_placeholder(&default_instance_id)
                                    .with_help_message("The status of the DB instance should be \"stopped\"; otherwise, this operation will result in a panic (the Rust way of handling runtime exceptions).")
                                    .prompt_skippable()
                                    .unwrap()
                                    .unwrap();
                        match db_instance_identifier.is_empty() {
                                false => {
                                
                                    rds_ops
                                        .start_db_instance(Some(&db_instance_identifier))
                                        .await
                                }
                                true => rds_ops.start_db_instance(None).await,
                            }
                        }

                        "Stop Db Instance\n" => {
                            let default_instance_id = format!(
                                "The default instance ID: {}\n",
                                rds_ops.get_db_instance_id()
                            );
                            let db_instance_identifier = Text::new("Enter the database instance identifier for which you want to stop temporarily\n")  
                            .with_placeholder(&default_instance_id)
                            .with_help_message("The status of the DB instance should be \"available\"; otherwise, this operation will result in a panic (the Rust way of handling runtime exceptions).")
                            .prompt_skippable()
                            .unwrap()
                            .unwrap();
                       match db_instance_identifier.is_empty() {
                                false => rds_ops.stop_db_instance(Some(&db_instance_identifier)).await,
                                true => rds_ops.stop_db_instance(None).await,
                            }
                        }
                 "Modify Database Instance Settings\n" => {

                    let db_instance_identifier = Text::new("Enter the DB instance ID you wish to modify\n")
                                .with_placeholder("You can modify only master password\n")
                                .prompt()
                                .unwrap();
                    let master_password = Text::new("Enter the new master password to replace the old one\n")
                                    .with_placeholder("Please remember this password, as it is used to connect to various database instances\n",)
                                    .prompt()
                                    .unwrap();
                    let apply = Confirm::new("Would you like to apply the changes immediately, or would you prefer to have Amazon Web Services do it later?\n")
                                .with_placeholder("Select 'Yes' to apply immediately or 'No' to have it done later by AWS")
                                .prompt()
                                .unwrap();
                      match  (db_instance_identifier.is_empty(),master_password.is_empty()){
                        (false,false) => rds_ops.modify_db_instance(&db_instance_identifier,&master_password,apply).await,
                        _ => println!("{}\n", "Fields cannot be left empty.".red().bold()),
                      }   
                      
                        }

                        "Delete Db Instance\n" => {
                            let default_instance_id = format!(
                                "The default instance ID: {}\n",
                                rds_ops.get_db_instance_id()
                            );
                            let db_instance_identifier = Text::new("Enter the database instance identifier you wish to delete permanently\n")  
                            .with_placeholder(&default_instance_id)
                            .prompt_skippable()
                            .unwrap()
                            .unwrap();
                             match db_instance_identifier.is_empty() {
                                false => rds_ops.delete_db_instance(Some(&db_instance_identifier)).await,
                    
                                true => rds_ops.delete_db_instance(None).await,
                            }
                        }

                     "Status of Db Instance\n" => {
                        let default_db_instance = format!("Default Db Instance Id: {}",rds_ops.get_db_instance_id());
                        let db_instance_identifier = Text::new("Enter the database instance identifier\n")  
                        .with_placeholder(&default_db_instance)
                        .prompt_skippable()
                        .unwrap()
                        .unwrap();

                    match db_instance_identifier.is_empty(){
                        false => rds_ops.status_of_db_instance(Some(&db_instance_identifier)).await,
                        true => rds_ops.status_of_db_instance(None).await
                    }

                     }

                        "Describe Db Cluster\n" => {
                            let default_cluster_id = format!(
                                "The default cluster ID: {}\n",
                                rds_ops.get_db_cluster_id()
                            );
                            let db_cluster_identifier = Text::new("Enter the database cluster identifier, which is different from the database instance identifier\n")  
                             .with_placeholder(&default_cluster_id)
                              .prompt_skippable()
                               .unwrap()
                               .unwrap();
                            let cluster_info = match db_cluster_identifier.is_empty() {
                                false => {
                                    rds_ops
                                        .describe_db_cluster(Some(&db_cluster_identifier))
                                        .await
                                }
                                true => rds_ops.describe_db_cluster(None).await,
                            };

                            cluster_info.into_iter().for_each(|dbclusterinfo| {
                                let colored_status = dbclusterinfo.get_status();
                                let instance_members = dbclusterinfo.get_db_members();
                                let colored_msg = "Status of Db Cluster: ".blue().bold();
                                println!("{}{:?}\n", colored_msg, colored_status);
                                println!("{}\n", "Db Instances info:".blue().bold());
                                instance_members.into_iter().for_each(|db_instance_info| {
                                    let colored_id = db_instance_info.green().bold();
                                    println!("{colored_id}\n");
                                })
                            });
                        }
                        "Delete Db Cluster\n" => {
                            let default_cluster_id = format!(
                                "The default cluster ID: {}\n",
                                rds_ops.get_db_cluster_id()
                            );
                            let db_cluster_identifier = Text::new("Enter the database cluster identifier, which is different from the database instance identifier\n")  
                             .with_placeholder(&default_cluster_id)
                             .prompt_skippable()
                             .unwrap()
                             .unwrap();
                            let cluster_info = match db_cluster_identifier.is_empty() {
                                false => {
                                    rds_ops
                                        .delete_db_cluster(Some(&db_cluster_identifier))
                                        .await
                                }
                                true => rds_ops.delete_db_cluster(None).await,
                            };

                            let colored_status = cluster_info
                                .get_status()
                                .map(|status| status.green().bold());
                            let instance_members = cluster_info.get_db_members();
                            println!("Status of Db Cluster: {:?}\n", colored_status);
                            println!("{}\n", "Db Instances info".blue().bold());
                            instance_members.into_iter().for_each(|db_instance_id| {
                                let colored_instance_id = db_instance_id.green().bold();
                                println!("Db Instance Id: {}\n", colored_instance_id);
                            });
                        }
                        "Go to main menu\n" => continue 'main,

                        _ => println!("Never reach"),
                    }
                }
            }

            "MemoryDb Operations\n" => {
                let memdb_choices = vec![
                    "Create MemDb Cluster\n",
                    "Create MemDb User\n",
                    "Describe MemDb Cluster\n",
                    "Describe MemDb User\n",
                    "Describe Snapshots of MemDb Cluster\n",
                    "Retrieve the database URL for connection\n",
                    "Delete MemDb User\n",
                    "Delete Cluster\n",
                    "Go To Main Menu\n",
                ];

                loop {
                    let choices =
                        Select::new("Select the operations to execute\n", memdb_choices.clone())
                            .prompt()
                            .unwrap();

                    match choices {
                        "Create MemDb Cluster\n" => {
                            let cluster_name = Text::new("Enter the cluster name\n")
                                .with_placeholder("The name must be uniquely identifiable")
                                .prompt()
                                .unwrap();
                            let possible_values = vec![
                                "db.t4g.small",
                                "db.r6g.large",
                                "db.r6g.xlarge",
                                "db.r6g.2xlarge",
                            ];
                            let possible_values =
                                format!("Some possible Values are: {:#?}\n", possible_values);
                            let node_type =
                                Text::new("Select the node type for your database system\n")
                                    .with_placeholder(&possible_values)
                                    .with_help_message(
                                        "look here to know more https://tinyurl.com/axy83wff",
                                    )
                                    .prompt()
                                    .unwrap();

                            let acl_name = Text::new("Specify the name of the Access Control List (ACL) to associate with the cluster\n")
                        .with_placeholder("Acl name is created through the aws console of memdb.")
                        .with_help_message("look here to know more https://tinyurl.com/yn3n4wya")
                        .prompt()
                        .unwrap();

                            match (
                                cluster_name.is_empty(),
                                node_type.is_empty(),
                                acl_name.is_empty(),
                            ) {
                                (false, false, false) => {
                                    memdb_ops
                                        .create_memdb_cluster(&node_type, &cluster_name, &acl_name)
                                        .await;
                                }
                                _ => println!("{}\n", "Fields cannot be left empty.".red().bold())
                            }
                        }
                "Create MemDb User\n" => {
                    let user_name = Text::new("Please provide a name for this MemDB user\n")
                        .with_placeholder("This name will also serve as the username for the database within a MemDB cluster\n")
                        .prompt()
                        .unwrap();
                    let possible_access_string_values = "The formats\n 'on' -The user is an active user\n '~*' - Access is given to all available keys\n '+@all' - Access is given to all available commands\n";
                    let access_string = Text::new("Please provide the access string or permission values for this user\n")
                                       .with_placeholder(possible_access_string_values)
                                       .with_help_message("Look here to know more https://tinyurl.com/2p9mnm64")
                                       .prompt()
                                       .unwrap();
                    let possible_authenticated_types = "    iam or Iam\n    Password or password\n"; 
                    let auth_type = Text::new("Specify the authenticated user's type\n")
                                    .with_placeholder(possible_authenticated_types)
                                    .with_help_message("Look here to know more https://tinyurl.com/3zaztx97")
                                    .prompt()
                                    .unwrap();
                    let passwords = Text::new("Please enter the passwords for the memdb user\n")
                                     .with_placeholder("Please remember this password; it's used for authenticating the database in a 'memdb' cluster")
                                     .with_help_message("Please ensure that your password contains a minimum of 16 characters")
                                     .prompt()
                                     .unwrap();
                  match (user_name.is_empty(),access_string.is_empty(),auth_type.is_empty(),passwords.is_empty()){
                    (false,false,false,false) => {
                        memdb_ops.create_memdb_user(&user_name,&access_string,&auth_type,&passwords).await;
                        let mut file = OpenOptions::new().create(true).write(true)
                                                 .read(true).open("./create_memdb_user_choices.txt").unwrap();
                            let choices = format!("Memdb User Name: {user_name}\nAccess String value: {access_string}\nAuthentication Type: {auth_type}\nPasswords: {passwords}\n");
                            
                           match file.write_all(choices.as_bytes()){
                               Ok(_) => {
                                let colored_msg ="The choices have been saved to the current directory for your reference\n".green().bold();
                                println!("{colored_msg}");
                               }
                               Err(_) => println!("Error while writting file to the current directory\n")
                            }

                    }
                    _ => println!("{}\n", "Fields cannot be left empty.".red().bold())

                  }                   
                }
                        
                "Describe MemDb Cluster\n" => {
                        let cluster_name = Text::new("Enter the cluster name for which you want to retrieve information\n")
                        .with_placeholder("The cluster anem is generated during the MemDB cluster creation process")
                        .prompt()
                        .unwrap();

                            match cluster_name.is_empty() {
                                false => {
                                    let info =
                                        memdb_ops.describe_memdb_cluster(&cluster_name).await;
                                    info.into_iter()
                                .for_each(|memclusterinfo|{
                                     let status = memclusterinfo.get_status().unwrap().green().bold();
                                     let acl_name = memclusterinfo.get_acl_name().unwrap().green().bold();
                                     println!("Status of MemdbCluster: {}\nAccess Control List(ACL) name: {}\n",status,acl_name);
                                });
                                }

                                true => println!(
                                    "{}\n",
                                    "The cluster name field can't be empty".red().bold()
                                ),
                            }
                        }
        "Describe MemDb User\n" => {
            let username = Text::new("Enter the MemDB user name for which you want to retrieve information\n")
             .with_placeholder("The username is generated during the MemDB user creation process")
            .prompt()
            .unwrap();
        match username.is_empty(){
            false => {
                let user_info = memdb_ops.describe_memdb_user(&username).await;
                let status = user_info[0].get_status().take();
                let access_string = user_info[0].get_access_string().take();
                println!("Status of User: {status:?}\n");
                println!("Access String for the User: {access_string:?}\n");
                user_info[0].print_auth_info();
            }
            true => println!("{}\n", "Fields cannot be left empty.".red().bold())
        }

        }

        "Describe Snapshots of MemDb Cluster\n" => {
                            let cluster_name = Text::new(
                                "Enter the cluster name for which you want to get snapshots\n",
                            )
                            .with_placeholder("The cluster name is generated during the MemDB cluster creation process.")
                            .prompt()
                            .unwrap();

                            match cluster_name.is_empty() {
                                false => {
                                    let snapshots =
                                        memdb_ops.describe_snapshots(&cluster_name).await;
                                    snapshots.into_iter()
                    .for_each(|snapshot|{
                        let snapshot_name = snapshot.name();
                        let snapshot_status = snapshot.status();
                        println!("Snapshot Name: {snapshot_name:?}\nStatus of snapshot: {snapshot_status:?}\n");
                    });
                                }
                                true => {
                                    println!("{}\n", "Cluster name can't be empty".red().bold())
                                }
                            }
                        }

                        "Retrieve the database URL for connection\n" => {
                            let cluster_name = Text::new("Enter the cluster name for which you need the connection URL\n")
                        .with_placeholder("The cluster name is the name assigned to the cluster when it was initially created")
                        .prompt()
                        .unwrap();

                            match cluster_name.is_empty() {
                                false => {
                                    let info = memdb_ops.describe_memdb_cluster(&cluster_name).await;
                                   let endpoint_with_port = info[0].get_endpoint_with_port();
                                   if let Some(endpoint_port) = endpoint_with_port{
                                    let redis_url =format!("redis://{endpoint_port}").green().bold();
                                    println!("The redis database url is: {redis_url}\n");
                                   }
                                }
                                true => println!(
                                    "{}\n",
                                    "MemdDb cluster name can't be empty".red().bold()
                                ),
                            }
                        }
            "Delete MemDb User\n" => {
            let username = Text::new("Enter the MemDB user name to delete\n") 
                          .with_placeholder("The username is generated during the MemDB user creation process")
                           .prompt()
                           .unwrap();
              match username.is_empty(){
                false => memdb_ops.delete_memdb_user(&username).await,
                true => println!("{}\n", "User name can't be empty".red().bold())
              }          
                
            }
                        "Delete Cluster\n" => {
                            let cluster_name =
                                Text::new("Enter the cluster name for which you want to delete\n")
                                     .with_placeholder("The cluster name is generated during the MemDB cluster creation process.")
                                    .prompt()
                                    .unwrap();
                            let final_snapshot_name =
                                Text::new("Create snapsho\n")
                                    .with_placeholder("You can create a final snapshot of your cluster before it’s deleted so you can restore it later")
                                    .prompt()
                                    .unwrap();

                            match (cluster_name.is_empty(), final_snapshot_name.is_empty()) {
                                (false, false) => {
                                    memdb_ops
                                        .delete_memdb_cluster(&cluster_name, &final_snapshot_name)
                                        .await;
                                }
                                _ => println!("{}\n", "Fields cannot be left empty".red().bold()),
                            }
                        }
                        "Go To Main Menu\n" => continue 'main,
                        _ => println!("Never reach"),
                    }
                }
            }

            "Quit the application\n" => {
                credential.empty();
                break 'main;
            }
            _other => {
                println!("This branch never reach..");
            }
        }
    }
}
fn global_render_config() -> RenderConfig {
    let mut config = RenderConfig::default()
        .with_prompt_prefix(Styled::new(">>> ").with_fg(inquire::ui::Color::DarkGreen))
        .with_text_input(StyleSheet::new().with_fg(inquire::ui::Color::LightGreen))
        .with_highlighted_option_prefix(Styled::new("👉").with_fg(inquire::ui::Color::DarkRed))
        .with_help_message(StyleSheet::new().with_fg(inquire::ui::Color::DarkYellow));
    config.error_message = config.error_message;
    config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(inquire::ui::Color::DarkGreen);
    config
}
