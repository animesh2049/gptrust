use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionPrompt { // oneOf
    prompt_text: Option<String>,
    prompt_texts: Option<Vec<String>>,
    prompt_list: Option<Vec<i64>>,
    prompt_lists: Option<Vec<Vec<i64>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StopWords { // oneOf
    stop_word: Option<String>,
    stop_words: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCompletionRequest {
    model: String,
    prompt: CompletionPrompt,
    suffix: String,
    max_tokens: i32,
    temperature: f64,
    top_p: f64,
    n: i32,
    stream: bool,
    logprobs: i32,
    echo: bool,
    stop: StopWords,
    presence_penalty: f64,
    frequency_penalty: f64,
    best_of: i32,
    logit_bias: HashMap,
    user: String,
}

impl CreateCompletionRequest {
    fn validate(&self) -> Result<bool, Box<dyn std::error::Error>> {
        if model == "" {
            return Err("Please provide a model to use for the completion");
        }
        let prompt_count = (
            self.prompt.prompt_text.is_some() as i32 +
            self.prompt.promtp_texts.is_some() as i32 +
            self.prompt.prompt_list.is_some() as i32 +
            self.prompt.prompt_lists.is_some() as i32
        );
        if prompt_count != 1 {
            return Err("You can pass only one type of prompt!");
        }

        let stop_word_count = (
            self.stop.stop_word.is_some() as i32 +
            self.stop.stop_words.is_some() as i32
        );
        if stop_word_count != 1 {
            return Err("You can pass either a stop word or a list of stop words, not both!");
        }
        if self.stop.stop_word.is_some() {
            return Ok(true);
        }
        if self.stop.stop_words.is_some() && self.stop.stop_words.len() <= 4 {
            return Err("You can pass maximum 4 stop words");
        }
        Ok(true)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogProbs {
    tokens: Vec<String>,
    token_log_probs: Vec<f64>,
    top_log_probs: Vec<HashMap>, // couldn't understand specific type from the openapi yaml
    text_offset: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    text: String,
    index: i32,
    log_probs: LogProbs,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    prompt_tokens: i32,
    completed_tokens: i32,
    total_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCompletionResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

pub async create_completion(req: CreateCompletionRequest) -> Result<CreateCompletionResponse, Box<dyn std::error::Error>> {
    let request_body = serde_json.to_string(req);
    let resp_body = gptrust_http::openai_http::openai_get("completions".to_string(), request_body);
    let completion_resp: CreateCompletionResponse = serde_json::from_str(&resp_body)?;
    Ok(completion_resp)
}
