fn main() {
    let test: &str = "El éxito no se mide por lo que logras, sino por los obstáculos";
    let bloque: [[u32; 4]; 4] = chacha(String::from("contraseñaDe32Caracteres"), 0);

    let x:Vec<char> = test.chars().collect();
    let y = x.iter().map(|&c| c as u32).collect();

    let resultado = sumar(y, bloque);
    let resultado2 = sumar(resultado.clone(), bloque);

    for i in resultado {
        print!("{i}");     
    }
    println!("");
    for i in resultado2 {
        println!("{i}");     
    }
   
}

fn chacha(mut key: String, counter: u32) -> [[u32; 4]; 4] {
    let expand_key: &[u8] = "er4un50nid0frioK".as_bytes();
    let nonce: [u32; 3] = [194057 as u32, 22709 as u32, 42157 as u32];

    if key.len() < 32 {
        let tam = 32 - key.len();
        for _ in 0..tam {
            key += "*";
        }
    }

    let key_char: &[u8] = key.as_bytes();

    let mut matrix: [[u32; 4]; 4] = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];

    // llenar con el expand key la matriz

    for i in 0..4 {
        matrix[0][i] = expand_key[i * 4] as u32
            + expand_key[i * 4 + 1] as u32
            + expand_key[i * 4 + 2] as u32
            + expand_key[i * 4 + 3] as u32;
    }

    // llenar la key en la matriz

    for j in 1..3 {
        for i in 0..4 {
            matrix[j][i] = key_char[i * 4 * j] as u32
                + key_char[i * 4 * j + 1] as u32
                + key_char[i * 4 * j + 2] as u32
                + key_char[i * 4 * j + 3] as u32;
        }
    }

    // llenar counter & nonce
    // generar números aleatorios random.gen_range(0..4294967296 as i64) as u32;
    matrix[3][0] = counter;
    matrix[3][1] = nonce[0];
    matrix[3][2] = nonce[1];
    matrix[3][3] = nonce[2];

    for _ in 0..20 {
        // Realizar las 20 rondas
        for i in 0..4 {
            horizontal(i, &mut matrix);
        }

        for i in 0..4 {
            diagonal(i, &mut matrix);
        }
    }

    matrix
}

fn horizontal(level: usize, matrix: &mut [[u32; 4]; 4]) {
    let (a, b) = matrix.split_at_mut(1);
    let (b, c) = b.split_at_mut(1);
    let (c, d) = c.split_at_mut(1);
    let (d, _) = d.split_at_mut(1);

    quarter_round(
        &mut a[0][level],
        &mut b[0][level],
        &mut c[0][level],
        &mut d[0][level],
    );
}

fn diagonal(level: usize, matrix: &mut [[u32; 4]; 4]) {
    let (a, b) = matrix.split_at_mut(1);
    let (b, c) = b.split_at_mut(1);
    let (c, d) = c.split_at_mut(1);
    let (d, _) = d.split_at_mut(1);

    quarter_round(
        &mut a[0][level % 4],
        &mut b[0][(level + 1) % 4],
        &mut c[0][(level + 2) % 4],
        &mut d[0][(level + 3) % 4],
    );
}

fn quarter_round(a: &mut u32, b: &mut u32, c: &mut u32, d: &mut u32) {
    // se debe realizar una ronda de ARX (adición, rotación y XOR)

    fn rotar(x: u32, n: u32) -> u32 {
        return (x << n) | (x >> (32 - n)) as u32;
    }

    *a = a.wrapping_add(*b);
    *d ^= *a;
    rotar(*d, 16);

    *c = c.wrapping_add(*d);
    *b ^= *c;
    rotar(*b, 12);

    *a = a.wrapping_add(*b);
    *d ^= *a;
    rotar(*d, 8);

    *c = c.wrapping_add(*d);
    *b ^= *c;
    rotar(*b, 7);
}

fn sumar(plain: Vec<u32>, bloque: [[u32; 4]; 4]) -> Vec<u32>{
    let mut contador: usize = 0;
    let mut cypher:Vec<u32> = Vec::new();
    for i in bloque {
        for j in i {
            cypher.push(plain[contador] as u32 ^ j);
            contador += 1;
        }
    }
   cypher
}


#[allow(dead_code)]
fn imprimir_matriz(matriz: [[u32; 4]; 4]) {
    for i in matriz {
        for j in i {
            print!("{j} ");
        }
        println!(" ")
    }
    println!("------------------------");
}
