pub fn is_comment(line: &str) -> bool {
    line.starts_with("/") 
    || line.starts_with("*")
}

pub fn is_using(line: &str) -> bool {
    line.starts_with("using")
}

pub fn is_namespace(line: &str) -> bool {
    line.starts_with("namespace")
}

pub fn is_class(line: &str) -> bool {
    line.starts_with("public class")
}

pub fn add_class(line: &str, lines: &mut Vec<String>) {
    lines.push("\n\n".to_string());

    // public class FavoriteShopDTO :  IEquatable<FavoriteShopDTO>, IValidatableObject
    let parts: Vec<String> = line.split(':').map(|x| x.to_string()).collect();
    let class = parts[0].to_owned();

    let record = class.replace("class", "sealed record");

    lines.push(record.to_string());
    lines.push("\n{".to_string());
}

pub fn is_property_decorator(line: &str) -> bool {
    line.contains("DataMember")
}

pub fn add_property_decorator(line: &str, lines: &mut Vec<String>) {
    // [DataMember(Name="json_name", EmitDefaultValue=false)]
    // 123456789012 -> 12
    let mut name_emit: Vec<char> = line.chars().skip(12).collect();
    
    // Name="json_name", EmitDefaultValue=false)]
    name_emit.pop();
    name_emit.pop();
    //let name_emit: String = name_emit.iter().take(name_emit.len() - 2).map(|&x| x).collect();
    
    // Name="json_name", EmitDefaultValue=false
    let name_emit: String = name_emit.iter().collect();
    let name_emit: Vec<String> = name_emit.split(',').map(|x| x.to_string()).collect();

    // Name="json_name"
    // 123456
    let mut name: Vec<char> = name_emit[0].chars().skip(6).collect();
    
    // json_name"
    name.pop();
    let name: String = name.iter().collect();

    // json_name
    let decorator = format!("\n\t[JsonPropertyName(\"{}\")]", name);
    lines.push(decorator);
}

pub fn is_property(line: &str) -> bool {
    line.ends_with("{ get; set; }")
}

pub fn add_property(line: &str, lines: &mut Vec<String>) {
    // public type? name { get; set; }
    let property_splitted: Vec<String> = line.split(' ').map(|x| x.to_string()).collect();

    let mut property_type: String = property_splitted.iter().skip(1).take(1).map(|x| x.to_string()).collect();
    if property_type.to_lowercase().contains("enum")
        || property_type == "long".to_string() {
        property_type = "int".to_string();
    }
    if !property_type.contains("?") {
        property_type = format!("{}?",property_type);
        //property_type.replace("?", "");
    }

    let property_name: String = property_splitted.iter().skip(2).take(1).map(|x| x.to_string()).collect();

    let property = format!("\n\tpublic {property_type} {property_name} {{ get; init; }}");

    lines.push(property);
}

pub fn close_braces(qty: i32, lines: &mut Vec<String>) {
    for _i in 0..qty {
        lines.push("\n}".to_string());
    }
}