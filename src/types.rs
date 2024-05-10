#[derive(Debug)]
pub enum ImapDumper {

    Authenticationfailed(String),
    ErrorBuilder(String),
    AuthenticationSucces(String),
}
#[derive(Debug)]
pub enum Search{
    Encontrou(String),
    NotEncontrou,
    PoucasMensagens,
}