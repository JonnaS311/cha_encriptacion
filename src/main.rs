use std::fs;
mod cka;

fn main() -> Result<(), std::io::Error> {
    //let test: &str = "El éxito no se mide por lo que logras, sino por los obstáculos";

    let data: Vec<u8> = fs::read("./imagen.jpg")?;
    let to_32: Vec<u32> = data.iter().map(|&c| c as u32).collect();

    let mut cifrador = cka::ChcaCha20 {
        counter: 0
    };

    let datos_cifrados = cifrador.cifrar(to_32.clone(), String::from("unaLlaveSimple"));

    let datos_resueltos:Vec<u32> = cifrador.cifrar(datos_cifrados, String::from("unaLlaveSimple"));

     // Convertir el Vec<u32> en Vec<u8>
     let byte_data: Vec<u8> = datos_resueltos.iter()
     .map(|&num| num as u8) // Convertir cada u32 en un array de 4 bytes
     .collect();

    fs::write("./solución.png", byte_data)?;

    Ok(())
}
