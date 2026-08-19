import fs from 'fs';
let code = fs.readFileSync('src/Effect/Console.rs', 'utf8');

code = code.replace(/pub fn (.*?)\(mut a0: UnknownType\)/g, "pub fn $1(mut a0: String)");
code = code.replace(/a0\.init_string\.as_ref\(\)\.unwrap\(\)/g, "a0");

fs.writeFileSync('src/Effect/Console.rs', code);
