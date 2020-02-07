use std::path::Path;
use std::env;
use std::sync::Arc;
use rust_transformers::preprocessing::vocab::base_vocab::Vocab;
use rust_transformers::bert_tokenizer::BertTokenizer;
use rust_transformers::preprocessing::tokenizer::base_tokenizer::{Tokenizer, TruncationStrategy};
use tch::{Device, nn, Tensor};

use crate::distilbert::distilbert::{DistilBertConfig, DistilBertModel};

mod distilbert;

fn main() {

//    Config & set-up var store
    let config_path = env::var("distilbert_config_path").unwrap();
    let config_path = Path::new(&config_path);
    let config = DistilBertConfig::from_file(config_path);
//    println!("{:?}", config);
//    let device = Device::cuda_if_available();
    let device = Device::Cpu;
    let vs = nn::VarStore::new(device);

//    Creation of tokenizer
    let vocab_path = "E:/Coding/backup-rust/rust-transformers/resources/vocab/bert-base-uncased-vocab.txt";
    let vocab = Arc::new(rust_transformers::BertVocab::from_file(vocab_path));
    let tokenizer: BertTokenizer = BertTokenizer::from_existing_vocab(vocab.clone());

//    Creation of sample input for testing purposes
    let input = ["Hello, world! This is a tokenization test", "This is the second sentence", "And a third sentence that is a bit longer"];
    let tokenized_input = tokenizer.encode_list(input.to_vec(), 128, &TruncationStrategy::LongestFirst, 0);
    let max_len = tokenized_input.iter().map(|input| input.token_ids.len()).max().unwrap();
    let tokenized_input = tokenized_input.
        iter().
        map(|input| input.token_ids.clone()).
        map(|mut input| {
            input.extend(vec![0; max_len - input.len()]);
            input
        }).
        map(|input|
            Tensor::of_slice(&(input))).
        collect::<Vec<_>>();
    let input_tensor = Tensor::stack(tokenized_input.as_slice(), 0).to(device);


    let distil_bert_model = DistilBertModel::new(&vs.root(), &config);
    let output = distil_bert_model.forward_t(Some(input_tensor), None, None, false);
    let (output, all_hidden_states, all_attentions) = output.unwrap();
    println!("{:?}", output);
    println!("{:?}", all_hidden_states);
    println!("{:?}", all_attentions);


//ToDo: check if the input is always padded to max_seq_length
}
