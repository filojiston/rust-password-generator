use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author="gencseyitahmet", version, about="now you can generate secure passwords from your terminal")]
pub struct Arguments {
    /// minimum length of the password
    #[clap(long, default_value_t = 8)]
    pub min_length: u8,
    /// maximum length of the password
    #[clap(long, default_value_t = 16)]
    pub max_length: u8,
    /// don't use punctuations in password (not recommended)
    #[clap(long)]
    pub no_punc: bool,
    /// don't use digits in password (not recommended)
    #[clap(long)]
    pub no_digit: bool,
}
