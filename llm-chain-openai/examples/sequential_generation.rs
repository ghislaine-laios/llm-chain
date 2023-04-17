use llm_chain::{chains::sequential::Chain, prompt};
use llm_chain_openai::chatgpt::{Executor, Step};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Create a new ChatGPT executor with the default settings
    let exec = Executor::new_default();

    // Create a chain of steps with two prompts
    let chain = Chain::new(vec![
        // First step: make a personalized birthday email
        Step::for_prompt(
            prompt!("You are a bot for making personalized greetings", "Make personalized birthday e-mail to the whole company for {{name}} who has their birthday on {{date}}. Include their name")
        ),

        // Second step: summarize the email into a tweet. Importantly, the text parameter becomes the result of the previous prompt.
        Step::for_prompt(
            prompt!( "You are an assistant for managing social media accounts for a company", "Summarize this email into a tweet to be sent by the company, use emoji if you can. \n--\n{{text}}")
        )
    ]);

    // Run the chain with the provided parameters
    let res = chain
        .run(
            // Create a Parameters object with key-value pairs for the placeholders
            vec![("name", "Emil"), ("date", "February 30th 2023")].into(),
            &exec,
        )
        .await
        .unwrap();

    // Print the result to the console
    println!("{:?}", res);
}