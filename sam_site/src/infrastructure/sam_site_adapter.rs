use std::collections::HashMap;

use reqwest::Client;

use students::domain::entities::{MusicianLevel, OrganistLevel, Region, Student, StudentPosition};

#[derive(serde::Deserialize, Debug)]
struct StudentResponseJson {
    draw: Option<String>,
    recordsTotal: u32,
    recordsFiltered: u32,
    data: Vec<Vec<Option<String>>>,
}

impl StudentResponseJson {
    fn to_entity(&self) -> Vec<Student> {
        self.data.iter().map(parse_student).collect()
    }
}

pub fn create_client() -> Client {
    Client::builder()
        // .redirect(Policy::none())
        .cookie_store(true)
        .build()
        .unwrap()
}

pub async fn get_session_id(client: &Client) {
    let mut form = HashMap::new();
    form.insert("login", "prism1095@duck.com");
    form.insert("password", "1glsWRzDoO");

    client
        .post("https://musical.congregacao.org.br/autenticar")
        .form(&form)
        .send()
        .await
        .unwrap();

    // let session_id = response.headers().get("set-cookie");

    client
        .get("https://musical.congregacao.org.br/alunos")
        .send()
        .await
        .unwrap();

    // session_id.unwrap().to_str().unwrap().to_owned()
}

pub async fn get_students(client: &Client) {
    let mut form = HashMap::new();
    // form.insert("draw", "1");
    // form.insert("columns[0][data]", "0");
    // form.insert("columns[0][name]", "");
    // form.insert("columns[0][searchable]", "false");
    // form.insert("columns[0][orderable]", "false");
    // form.insert("columns[0][search][value]", "");
    // form.insert("columns[0][search][regex]", "false");
    // form.insert("columns[1][data]", "1");
    // form.insert("columns[1][name]", "");
    // form.insert("columns[1][searchable]", "true");
    // form.insert("columns[1][orderable]", "true");
    // form.insert("columns[1][search][value]", "");
    // form.insert("columns[1][search][regex]", "false");
    // form.insert("columns[2][data]", "2");
    // form.insert("columns[2][name]", "");
    // form.insert("columns[2][searchable]", "true");
    // form.insert("columns[2][orderable]", "true");
    // form.insert("columns[2][search][value]", "");
    // form.insert("columns[2][search][regex]", "false");
    // form.insert("columns[3][data]", "3");
    // form.insert("columns[3][name]", "");
    // form.insert("columns[3][searchable]", "true");
    // form.insert("columns[3][orderable]", "true");
    // form.insert("columns[3][search][value]", "");
    // form.insert("columns[3][search][regex]", "false");
    // form.insert("columns[4][data]", "4");
    // form.insert("columns[4][name]", "");
    // form.insert("columns[4][searchable]", "true");
    // form.insert("columns[4][orderable]", "true");
    // form.insert("columns[4][search][value]", "");
    // form.insert("columns[4][search][regex]", "false");
    // form.insert("columns[5][data]", "5");
    // form.insert("columns[5][name]", "");
    // form.insert("columns[5][searchable]", "true");
    // form.insert("columns[5][orderable]", "true");
    // form.insert("columns[5][search][value]", "");
    // form.insert("columns[5][search][regex]", "false");
    // form.insert("columns[6][data]", "6");
    // form.insert("columns[6][name]", "");
    // form.insert("columns[6][searchable]", "true");
    // form.insert("columns[6][orderable]", "false");
    // form.insert("columns[6][search][value]", "");
    // form.insert("columns[6][search][regex]", "false");
    // form.insert("order[0][column]", "0");
    // form.insert("order[0][dir]", "asc");
    form.insert("start", "0");
    form.insert("length", "999999999");
    form.insert("search[value]", "");
    form.insert("search[regex]", "false");

    let response = client
        .post("https://musical.congregacao.org.br/alunos/listagem")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("Referer", "https://musical.congregacao.org.br/alunos")
        .form(&form)
        .send()
        .await
        .unwrap();

    println!("Status: {}", response.status());

    // println!("Body: {:?}", response.text().await.unwrap());
    let json = response.json::<StudentResponseJson>().await.unwrap();
    println!("Json: {:?}", json.data.iter().map(|e| e.get(6)));
    println!("Len: {:?}", json.data.len());
    println!("Len inner: {:?}", json.data.get(10).unwrap().len());
    println!("Parsed: {:?}", json.to_entity());
}

fn parse_student(raw_data: &Vec<Option<String>>) -> Student {
    let id: String = raw_data.get(0).unwrap().clone().unwrap().to_owned();
    let name: String = raw_data.get(1).unwrap().clone().unwrap().to_owned();
    let (location, region) = parse_location_and_region(remove_double_or_more_spaces(
        remove_span_tags(raw_data.get(2).unwrap().clone().unwrap()),
    ));
    let raw_position: String = raw_data.get(3).unwrap().clone().unwrap().to_owned();
    let raw_level: String = raw_data.get(5).unwrap().clone().unwrap().to_owned();
    let position = parse_student_positon(raw_position, raw_level);

    Student {
        id,
        name,
        location,
        region,
        position,
    }
}

fn parse_location_and_region(value: String) -> (String, Region) {
    let mut parts = value.split('|');
    let location = parts.next().unwrap().to_owned();
    let raw_region = parts.next().unwrap();

    (location, parse_region(raw_region))
}

fn parse_region(value: &str) -> Region {
    match value {
        "BR-SP-ARARAQUARA-SÃO CARLOS" => Region::AraraquaraSaoCarlos,
        "BR-SP-ARARAQUARA-ITIRAPINA" => Region::AraraquaraItirapina,
        _ => Region::Other(value.to_owned()),
    }
}

fn parse_student_positon(raw_position: String, raw_level: String) -> StudentPosition {
    match raw_position.to_uppercase().as_str() {
        "MÚSICO" => StudentPosition::Musician {
            level: parse_musician_level(raw_level),
        },

        "ORGANISTA" => StudentPosition::Organist {
            level: parse_organist_level(raw_level),
        },
        other => StudentPosition::Unknown(other.to_owned()),
    }
}

fn parse_musician_level(value: String) -> MusicianLevel {
    match value.to_uppercase().as_str() {
        "CANDIDATO(A)" => MusicianLevel::Candidate,
        "CULTO OFICIAL" => MusicianLevel::OfficialService,
        "ENSAIO" => MusicianLevel::Practice,
        "RJM" => MusicianLevel::YouthService,
        other => MusicianLevel::Unknown(other.to_owned()),
    }
}
fn parse_organist_level(value: String) -> OrganistLevel {
    match value.to_uppercase().as_str() {
        "CANDIDATO(A)" => OrganistLevel::Candidate,
        "CULTO OFICIAL" => OrganistLevel::OfficialService,
        "ENSAIO" => OrganistLevel::Practice,
        "RJM" => OrganistLevel::YouthService,
        "MEIA HORA" => OrganistLevel::HafHour,
        "RJM / CULTO OFICIAL" => OrganistLevel::YouthServiceOfficialService,
        "RJM / ENSAIO" => OrganistLevel::YouthServicePractice,
        "RJM / MEIA HORA" => OrganistLevel::YouthServiceHafHour,
        "RJM / OFICIALIZADO(A)" => OrganistLevel::YouthServiceOfficialized,
        other => OrganistLevel::Unknown(other.to_owned()),
    }
}

fn remove_span_tags(input: String) -> String {
    let re: regex::Regex = regex::Regex::new(r"<span[^>]*></span>").unwrap();

    re.replace_all(input.as_str(), "").to_string()
}

fn remove_double_or_more_spaces(input: String) -> String {
    let re: regex::Regex = regex::Regex::new(r"(\s\s)*").unwrap();

    re.replace_all(input.as_str(), "").to_string()
}
