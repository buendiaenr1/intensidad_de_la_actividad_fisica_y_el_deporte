use nalgebra::{DMatrix, Scalar};

// enrique ricardo pablo buendia lozada
// 2023
// BUAP Facultad de Cultura Física

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[warn(unused_imports)]
fn rem() -> std::io::Result<()> {
    // borrar archivos anteriores , para tratar de disminuir errores
    fs::remove_file("dat1.csv")?;
    fs::remove_file("dat2.csv")?;
    fs::remove_file("dat3.csv")?;
    fs::remove_file("dat4.csv")?;
    fs::remove_file("dat5.csv")?;
    fs::remove_file("dat6.csv")?;
    fs::remove_file("dat7.csv")?;
    fs::remove_file("dat8.csv")?;
    fs::remove_file("dat9.csv")?;
    fs::remove_file("dat10.csv")?;
    fs::remove_file("dat11.csv")?;
    fs::remove_file("dat12.csv")?;
    fs::remove_file("dat13.csv")?;
    fs::remove_file("dat14.csv")?;
    fs::remove_file("dat15.csv")?;
    fs::remove_file("dat16.csv")?;
    fs::remove_file("dat17.csv")?;
    fs::remove_file("dat18.csv")?;
    fs::remove_file("dat19.csv")?;
    fs::remove_file("dat20.csv")?;
    fs::remove_file("dat21.csv")?;
    fs::remove_file("dat22.csv")?;
    fs::remove_file("dat23.csv")?;
    fs::remove_file("dat24.csv")?;
    fs::remove_file("dat25.csv")?;
    fs::remove_file("dat26.csv")?;
    fs::remove_file("dat27.csv")?;
    fs::remove_file("dat28.csv")?;
    fs::remove_file("dat29.csv")?;
    fs::remove_file("dat30.csv")?;
    fs::remove_file("dat31.csv")?;
    fs::remove_file("dat32.csv")?;
    fs::remove_file("dat33.csv")?;
    fs::remove_file("dat34.csv")?;
    fs::remove_file("dat35.csv")?;
    fs::remove_file("dat36.csv")?;
    fs::remove_file("dat37.csv")?;
    fs::remove_file("dat38.csv")?;
    fs::remove_file("dat39.csv")?;
    fs::remove_file("dat40.csv")?;
    fs::remove_file("dat41.csv")?;
    fs::remove_file("dat42.csv")?;
    fs::remove_file("dat43.csv")?;
    fs::remove_file("dat44.csv")?;
    fs::remove_file("dat45.csv")?;
    fs::remove_file("dat46.csv")?;
    fs::remove_file("dat47.csv")?;
    fs::remove_file("dat48.csv")?;
    fs::remove_file("dat49.csv")?;
    fs::remove_file("dat50.csv")?;
    fs::remove_file("dat51.csv")?;
    fs::remove_file("dat52.csv")?;
    fs::remove_file("dat53.csv")?;
    fs::remove_file("dat54.csv")?;
    fs::remove_file("dat55.csv")?;
    fs::remove_file("dat56.csv")?;
    fs::remove_file("dat57.csv")?;
    fs::remove_file("dat58.csv")?;
    fs::remove_file("dat59.csv")?;
    fs::remove_file("dat60.csv")?;
    fs::remove_file("dat61.csv")?;
    fs::remove_file("dat62.csv")?;
    fs::remove_file("dat63.csv")?;
    fs::remove_file("dat64.csv")?;
    fs::remove_file("dat65.csv")?;
    fs::remove_file("dat66.csv")?;
    fs::remove_file("dat67.csv")?;
    fs::remove_file("dat68.csv")?;
    fs::remove_file("dat69.csv")?;
    fs::remove_file("dat70.csv")?;
    fs::remove_file("dat71.csv")?;
    fs::remove_file("dat72.csv")?;
    fs::remove_file("dat73.csv")?;
    fs::remove_file("dat74.csv")?;
    fs::remove_file("dat75.csv")?;
    fs::remove_file("dat76.csv")?;
    fs::remove_file("dat77.csv")?;
    fs::remove_file("dat78.csv")?;
    fs::remove_file("dat79.csv")?;
    fs::remove_file("dat80.csv")?;
    fs::remove_file("dat81.csv")?;
    fs::remove_file("dat82.csv")?;
    fs::remove_file("dat83.csv")?;
    fs::remove_file("dat84.csv")?;
    fs::remove_file("dat85.csv")?;
    fs::remove_file("dat86.csv")?;
    fs::remove_file("dat87.csv")?;
    fs::remove_file("dat88.csv")?;
    fs::remove_file("dat89.csv")?;
    fs::remove_file("dat90.csv")?;
    fs::remove_file("dat91.csv")?;
    fs::remove_file("dat92.csv")?;
    fs::remove_file("dat93.csv")?;
    fs::remove_file("dat94.csv")?;
    fs::remove_file("dat95.csv")?;
    fs::remove_file("dat96.csv")?;
    fs::remove_file("dat97.csv")?;
    fs::remove_file("dat98.csv")?;
    fs::remove_file("dat99.csv")?;
    fs::remove_file("dat100.csv")?;
    fs::remove_file("dat101.csv")?;
    fs::remove_file("dat102.csv")?;
    fs::remove_file("dat103.csv")?;
    fs::remove_file("dat104.csv")?;
    fs::remove_file("dat105.csv")?;

    fs::remove_file("datt1.csv")?;
    fs::remove_file("datt2.csv")?;
    fs::remove_file("datt3.csv")?;
    fs::remove_file("datt4.csv")?;
    fs::remove_file("datt5.csv")?;
    fs::remove_file("datt6.csv")?;
    fs::remove_file("datt7.csv")?;
    fs::remove_file("datt8.csv")?;
    fs::remove_file("datt9.csv")?;
    fs::remove_file("datt10.csv")?;
    fs::remove_file("datt11.csv")?;
    fs::remove_file("datt12.csv")?;
    fs::remove_file("datt13.csv")?;
    fs::remove_file("datt14.csv")?;
    fs::remove_file("datt15.csv")?;
    fs::remove_file("datt16.csv")?;
    fs::remove_file("datt17.csv")?;
    fs::remove_file("datt18.csv")?;
    fs::remove_file("datt19.csv")?;
    fs::remove_file("datt20.csv")?;
    fs::remove_file("datt21.csv")?;
    fs::remove_file("datt22.csv")?;
    fs::remove_file("datt23.csv")?;
    fs::remove_file("datt24.csv")?;
    fs::remove_file("datt25.csv")?;
    fs::remove_file("datt26.csv")?;
    fs::remove_file("datt27.csv")?;
    fs::remove_file("datt28.csv")?;
    fs::remove_file("datt29.csv")?;
    fs::remove_file("datt30.csv")?;
    fs::remove_file("datt31.csv")?;
    fs::remove_file("datt32.csv")?;
    fs::remove_file("datt33.csv")?;
    fs::remove_file("datt34.csv")?;
    fs::remove_file("datt35.csv")?;
    fs::remove_file("datt36.csv")?;
    fs::remove_file("datt37.csv")?;
    fs::remove_file("datt38.csv")?;
    fs::remove_file("datt39.csv")?;
    fs::remove_file("datt40.csv")?;
    fs::remove_file("datt41.csv")?;
    fs::remove_file("datt42.csv")?;
    fs::remove_file("datt43.csv")?;
    fs::remove_file("datt44.csv")?;
    fs::remove_file("datt45.csv")?;
    fs::remove_file("datt46.csv")?;
    fs::remove_file("datt47.csv")?;
    fs::remove_file("datt48.csv")?;
    fs::remove_file("datt49.csv")?;
    fs::remove_file("datt50.csv")?;
    fs::remove_file("datt51.csv")?;
    fs::remove_file("datt52.csv")?;
    fs::remove_file("datt53.csv")?;
    fs::remove_file("datt54.csv")?;
    fs::remove_file("datt55.csv")?;
    fs::remove_file("datt56.csv")?;
    fs::remove_file("datt57.csv")?;
    fs::remove_file("datt58.csv")?;
    fs::remove_file("datt59.csv")?;
    fs::remove_file("datt60.csv")?;
    fs::remove_file("datt61.csv")?;
    fs::remove_file("datt62.csv")?;
    fs::remove_file("datt63.csv")?;
    fs::remove_file("datt64.csv")?;
    fs::remove_file("datt65.csv")?;
    fs::remove_file("datt66.csv")?;
    fs::remove_file("datt67.csv")?;
    fs::remove_file("datt68.csv")?;
    fs::remove_file("datt69.csv")?;
    fs::remove_file("datt70.csv")?;
    fs::remove_file("datt71.csv")?;
    fs::remove_file("datt72.csv")?;
    fs::remove_file("datt73.csv")?;
    fs::remove_file("datt74.csv")?;
    fs::remove_file("datt75.csv")?;
    fs::remove_file("datt76.csv")?;
    fs::remove_file("datt77.csv")?;
    fs::remove_file("datt78.csv")?;
    fs::remove_file("datt79.csv")?;
    fs::remove_file("datt80.csv")?;
    fs::remove_file("datt81.csv")?;
    fs::remove_file("datt82.csv")?;
    fs::remove_file("datt83.csv")?;
    fs::remove_file("datt84.csv")?;
    fs::remove_file("datt85.csv")?;
    fs::remove_file("datt86.csv")?;
    fs::remove_file("datt87.csv")?;
    fs::remove_file("datt88.csv")?;
    fs::remove_file("datt89.csv")?;
    fs::remove_file("datt90.csv")?;
    fs::remove_file("datt91.csv")?;
    fs::remove_file("datt92.csv")?;
    fs::remove_file("datt93.csv")?;
    fs::remove_file("datt94.csv")?;
    fs::remove_file("datt95.csv")?;
    fs::remove_file("datt96.csv")?;
    fs::remove_file("datt97.csv")?;
    fs::remove_file("datt98.csv")?;
    fs::remove_file("datt99.csv")?;
    fs::remove_file("datt100.csv")?;
    fs::remove_file("datt101.csv")?;
    fs::remove_file("datt102.csv")?;
    fs::remove_file("datt103.csv")?;
    fs::remove_file("datt104.csv")?;
    fs::remove_file("datt105.csv")?;

    Ok(())
}
// leer por lineas los datos y convertirlos a punto flotante
// para poder trabajar el algoritmo de IA de regresión lineal
fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut _rows = 0;

    rem();

    let nombre = "convert.csv";
    let mut file = File::create(nombre.clone()).expect("No se pueden crear archivos...");
    for line in reader.lines() {
        _rows += 1; //renglones leidos
        let mut linea: String = line?.clone();
        //println!("{} leidos {}", linea, rows);
        if linea.contains(",") {
            linea = linea.replace(",", ".0,");
            linea = linea.to_owned() + &".0";
            //println!("contiene ...{}", linea);
            writeln!(&mut file, "{}", linea).unwrap();
        }
    }

    Ok(())
}

//
// Leer datos convertidos a flotante e iniciar su tratamiento
// en formato matriz

fn parse_csv<N, R>(input: R) -> Result<DMatrix<N>, Box<dyn std::error::Error>>
where
    N: FromStr + Scalar,
    N::Err: std::error::Error,
    R: BufRead,
{
    // iniciar vector de información
    let mut data: Vec<N> = Vec::new();

    // contador de renglones del archivo
    let mut rows = 0;

    // para cada linea
    for line in input.lines() {
        // incrementar el numero de renglones
        rows += 1;
        // separar por comas cada renglon
        for datum in line?.split_terminator(",") {
            // limpiar de espacios la información
            data.push(N::from_str(datum.trim())?);
        }
    }

    // el numero de items dividido por los renglones es el numero de columnas
    let cols = data.len() / rows;

    // Construir una `DMatrix` del vector de información
    Ok(DMatrix::from_row_slice(rows, cols, &data[..]))
}

fn main() {
    println!("   ");
    println!("\n\n\n***** BUAP Facultad de Cultura Física\n\n");
    println!("***** Dr. Enrique R.P. Buendia Lozada  [buendiaenr1@gmail.com]\n\n");
    println!("***** Control de Intensidad de la actividad vía FC\n\n");

    read_file_line_by_line("datos.csv"); // los datos son enteros

    let file = File::open("convert.csv").unwrap(); // convertirlos a flotantes
    let bos: DMatrix<f64> = parse_csv(BufReader::new(file)).unwrap();
    println!(
        " General ---> CSV [5 renglones de muestra]: \n{}",
        bos.rows(0, 5)
    );

    let nr = bos.nrows();
    let pnr = (bos.nrows() as f64 * 0.8).round() as usize;
    let nc = bos.ncols();

    //println!(" rows  {}", nr);
    //println!(" colum {}", nc);
    //println!(" %rows  {}", pnr);

    // cintas de mediciones, buscando minimo y maximo de x
    let mut ndat = 0;

    // definir variables de trabajo
    let mut xxx: Vec<f64> = Vec::new();
    let mut yyy: Vec<f64> = Vec::new();
    let mut maximox = 0.0;
    let mut minimox: f64 = bos[(0, 0)];
    let mut maximoy = 0.0;
    let mut minimoy: f64 = 300.0; // no lo creo, pero sirve para esto

    // busca el minimo y el maximo , mientras copias los dos vectores
    for i in 0..nr {
        xxx.push(bos[(i, 0)]);
        if xxx[i] > maximox {
            maximox = xxx[i];
        }
        if xxx[i] <= minimox {
            minimox = xxx[i];
        }
        yyy.push(bos[(i, 1)]);
        if yyy[i] > maximoy {
            maximoy = yyy[i];
        }
        if yyy[i] <= minimoy && yyy[i] != 0.0 {
            minimoy = yyy[i];
        }
    }
    println!("x mínimo {} y máximo {} \n", minimox, maximox);
    println!("y mínimo {} y máximo {} \n", minimoy, maximoy);
    // opción 1:  de 0 a FCmaxima *****************************************************************
    let rep: usize = (maximoy / 6 as f64) as usize;
    let fraccion = 100.0 / rep as f64; // porcentaje en bajo de el
    println!(
        " ::: OPCIÓN 1: Fractiles : {} , las fracciones son del {}\n",
        rep, fraccion
    );
    for j in 1..rep {
        println!(
            " dat n . csv significa n={}, n*{}={}% de intensidad \n",
            j,
            fraccion,
            fraccion * j as f64
        );
    }
    // separar los datos por bandas de ancho 6
    // pues la ecuación de regresión lineal pasará por el promedio
    // lo que significa que los errores cumpliran estar en el rango pequeño de pulsaciones por minuto
    let mut vec0: Vec<f64> = Vec::new();
    let mut vec1: Vec<f64> = Vec::new();
    let mut csvf: Vec<String> = Vec::new();
    for j in 1..rep {
        for i in 0..nr {
            // multiplos de 6 es la deficion de las bandas
            if bos[(i, 1)] >= (6 * (j - 1)) as f64 && bos[(i, 1)] <= (6 * j) as f64 {
                vec0.push(bos[(i, 0)]);
                vec1.push(bos[(i, 1)]);
                ndat = ndat + 1;
            }
        }
        // si cont= tamaño de muestras representativas > 5 guardar csv para crear modelo
        if ndat >= 5 {
            let nom1 = "dat";
            let nom2 = j.to_string();
            let n3 = ".csv";
            let nombre = [nom1, &nom2, n3].join("");

            let mut file = File::create(nombre.clone()).expect("No se pueden crear archivos...");
            for thing in 0..vec0.len() {
                let n1 = vec0[thing].to_string();
                let n2 = vec1[thing].to_string();
                let rr = [n1, ",".to_string(), n2].join("");
                //file.write_all(&rr);
                writeln!(&mut file, "{}", rr).unwrap();
            }
            csvf.push(nombre);
        }
        //print!("\n cont {} {:?}\n", j, vec1);
        vec0.clear();
        vec1.clear();
        ndat = 0;
    }
    //for file in fs::read_dir("").unwrap() {
    //    println!("{}", file.unwrap().path().display());
    //}

    // nombres de archivos csv por cintas horizontales
    println!("\n{:?}\n", csvf);
    // **** FIN cintas de mediciones

    let variablesx = nc - 1;
    let x = bos.columns(0, variablesx).into_owned();
    let y = bos.column(variablesx).into_owned();

    //let (x_train, x_test, y_train, y_test) =
    //    train_test_split(&x, &y.transpose(), test_size = 0.2, shuffle = true);
    let x_train = x.rows(0, pnr as usize).into_owned();
    //println!("{}", x_train.rows(0, 5));
    //println!("  X entrenamiento \n {}", x_train.nrows());

    let x_test = x.remove_rows(0, pnr).into_owned();
    //println!("{}", x_test.rows(0, 5));
    //println!("  X de prueba \n {}", x_test.nrows());

    let y_train = y.rows(0, pnr as usize).into_owned();
    //println!("{}", y_train.rows(0, 5));
    //println!(" Y de entrenamiento \n {}", y_train.nrows());

    let y_test = y.remove_rows(0, pnr).into_owned();
    //let x_test= x.rows(pnr as usize, nr as usize).into_owned();
    //println!("{}", y_test.rows(0, 5));
    //println!("  Y de prueba \n {}", y_test.nrows());

    let a = x_train.clone().insert_column(variablesx, 1.0).into_owned();
    let b = y_train.clone().transpose();

    //-- Q, R = np.linalg.qr(A)
    // Q, R = np.linalg.qr(A)
    let qr = a.qr();
    let (q, r) = (qr.q().transpose(), qr.r());
    //let q=qr.q();
    //let r=qr.r();
    //println!("{}", r.rows(0, 5));
    let x = r.try_inverse().unwrap() * &q * &b.transpose();
    //println!("{}",x);
    let coeff = x.rows(0, variablesx);
    let intercept = x[(variablesx, 0)];
    let coe = coeff[0];
    println!(" fc = {} * edad + ({}) \n", coe, intercept);

    let y_hat = (x_test * &coeff).add_scalar(intercept);
    //println!("  y_estimada {}", y_hat.len());
    //println!("  y_prueba {}", y_test.len());
    //println!("mae: {}", mean_absolute_error(&y_test, &y_hat));
    let mut suma = 0.0;
    let mut suma2 = 0.0;
    for i in 0..y_test.len() {
        suma = suma + (y_test[i] - y_hat[i]).abs();
        suma2 = suma2 + (y_test[i] - y_hat[i]).powf(2.0);
    }
    let sumaf = suma.abs() / y_test.len() as f64;
    println!(" mae : {}", sumaf);
    let sumaf = suma2 / y_test.len() as f64;
    println!(" mse : {}", sumaf);
    let sumaf = (sumaf).powf(0.5);
    println!(" rmse: {}", sumaf);

    // ecuaciones por cintas horizontales
    for jf in csvf {
        let file = File::open(jf.clone()).unwrap();
        let bos: DMatrix<f64> = parse_csv(BufReader::new(file)).unwrap();
        //println!(
        //    " \n H---> CSV {} [5 renglones de muestra]: \n{}",
        //    jf,
        //    bos.rows(0, 5)
        //);
        println!(" \n H---> CSV {}  \n", jf);
        //let nr = bos.nrows();
        let pnr = (bos.nrows() as f64 * 0.8).round() as usize;
        let nc = bos.ncols();

        //println!(" rows  {}", nr);
        //println!(" colum {}", nc);
        //println!(" %rows  {}", pnr);
        let variablesx = nc - 1;
        let x = bos.columns(0, variablesx).into_owned();
        let y = bos.column(variablesx).into_owned();

        //let (x_train, x_test, y_train, y_test) =
        //    train_test_split(&x, &y.transpose(), test_size = 0.2, shuffle = true);
        let x_train = x.rows(0, pnr as usize).into_owned();
        //println!("{}", x_train.rows(0, 5));
        //println!("  X entrenamiento \n {}", x_train.nrows());

        let x_test = x.remove_rows(0, pnr).into_owned();
        //println!("{}", x_test.rows(0, 5));
        //println!("  X de prueba \n {}", x_test.nrows());

        let y_train = y.rows(0, pnr as usize).into_owned();
        //println!("{}", y_train.rows(0, 5));
        //println!(" Y de entrenamiento \n {}", y_train.nrows());

        let y_test = y.remove_rows(0, pnr).into_owned();
        //let x_test= x.rows(pnr as usize, nr as usize).into_owned();
        //println!("{}", y_test.rows(0, 5));
        //println!("  Y de prueba \n {}", y_test.nrows());

        let a = x_train.clone().insert_column(variablesx, 1.0).into_owned();
        let b = y_train.clone().transpose();

        //-- Q, R = np.linalg.qr(A)
        // Q, R = np.linalg.qr(A)
        let qr = a.qr();
        let (q, r) = (qr.q().transpose(), qr.r());
        //let q=qr.q();
        //let r=qr.r();
        //println!("{}", r.rows(0, 5));
        let x = r.try_inverse().unwrap() * &q * &b.transpose();
        //println!("{}",x);
        let coeff = x.rows(0, variablesx);
        let intercept = x[(variablesx, 0)];
        let coe = coeff[0];
        println!(" fc = {} * edad + ({}) \n", coe, intercept);

        let y_hat = (x_test * &coeff).add_scalar(intercept);
        //println!("  y_estimada {}", y_hat.len());
        //println!("  y_prueba {}", y_test.len());
        //println!("mae: {}", mean_absolute_error(&y_test, &y_hat));
        let mut suma = 0.0;
        let mut suma2 = 0.0;
        for i in 0..y_test.len() {
            suma = suma + (y_test[i] - y_hat[i]).abs();
            suma2 = suma2 + (y_test[i] - y_hat[i]).powf(2.0);
        }
        let sumaf = suma.abs() / y_test.len() as f64;
        println!(" mae : {}", sumaf);
        let sumaf = suma2 / y_test.len() as f64;
        println!(" mse : {}", sumaf);
        let sumaf = (sumaf).powf(0.5);
        println!(" rmse: {}", sumaf);
    }

    // **********************************************************************************
    // **********************************************************************************
    // opción de FCmínima a FCmaxima

    let rep: usize = ((maximoy - minimoy) / 6 as f64) as usize;
    let fraccion = 100.0 / rep as f64; // porcentaje en bajo de el
    println!(
        "\n ::: OPCIÔN 2: Fractiles : {} , las fracciones son del {}\n",
        rep, fraccion
    );
    for j in 1..rep {
        println!(
            " datt n . csv significa n={}, n*{}={}% de intensidad \n",
            j,
            fraccion,
            fraccion * j as f64
        );
    }

    vec0.clear();
    vec1.clear();
    let mut csvf2: Vec<String> = Vec::new();
    ndat = 0;
    let mut base: usize = 0;
    let mut equij: usize = 0;

    for j in 1..rep {
        for i in 0..nr {
            // multiplos de 6 es la deficion de las bandas
            if bos[(i, 1)] >= (6 * (j - 1)) as f64 + minimoy
                && bos[(i, 1)] < (6 * j) as f64 + minimoy
            {
                vec0.push(bos[(i, 0)]);
                vec1.push(bos[(i, 1)]);
                ndat = ndat + 1;
            }
            if ndat > 0 && base == 0 {
                base = j;
                equij = 1;
            }
        }
        // si cont= tamaño de muestras representativas > 10 guardar csv para crear modelo
        if ndat >= 5 && base > 0 {
            let nom1 = "datt";
            let nom2 = equij.to_string();
            let n3 = ".csv";
            let nombre = [nom1, &nom2, n3].join("");

            //println!(" j={} equij={}", j, equij);

            let mut file = File::create(nombre.clone()).expect("No se pueden crear archivos...");
            for thing in 0..vec0.len() {
                let n1 = vec0[thing].to_string();
                let n2 = vec1[thing].to_string();
                let rr = [n1, ",".to_string(), n2].join("");
                //file.write_all(&rr);
                writeln!(&mut file, "{}", rr).unwrap();
            }
            csvf2.push(nombre);
        }
        //print!("\n cont {} {:?}\n", j, vec1);
        vec0.clear();
        vec1.clear();
        ndat = 0;
        equij += 1;
    }
    //for file in fs::read_dir("").unwrap() {
    //    println!("{}", file.unwrap().path().display());
    //}

    // nombres de archivos csv por cintas horizontales
    println!("\n{:?}\n", csvf2);
    // **** FIN cintas de mediciones

    let variablesx = nc - 1;
    let x = bos.columns(0, variablesx).into_owned();
    let y = bos.column(variablesx).into_owned();

    //let (x_train, x_test, y_train, y_test) =
    //    train_test_split(&x, &y.transpose(), test_size = 0.2, shuffle = true);
    let x_train = x.rows(0, pnr as usize).into_owned();
    //println!("{}", x_train.rows(0, 5));
    //println!("  X entrenamiento \n {}", x_train.nrows());

    let x_test = x.remove_rows(0, pnr).into_owned();
    //println!("{}", x_test.rows(0, 5));
    //println!("  X de prueba \n {}", x_test.nrows());

    let y_train = y.rows(0, pnr as usize).into_owned();
    //println!("{}", y_train.rows(0, 5));
    //println!(" Y de entrenamiento \n {}", y_train.nrows());

    let y_test = y.remove_rows(0, pnr).into_owned();
    //let x_test= x.rows(pnr as usize, nr as usize).into_owned();
    //println!("{}", y_test.rows(0, 5));
    //println!("  Y de prueba \n {}", y_test.nrows());

    let a = x_train.clone().insert_column(variablesx, 1.0).into_owned();
    let b = y_train.clone().transpose();

    //-- Q, R = np.linalg.qr(A)
    // Q, R = np.linalg.qr(A)
    let qr = a.qr();
    let (q, r) = (qr.q().transpose(), qr.r());
    //let q=qr.q();
    //let r=qr.r();
    //println!("{}", r.rows(0, 5));
    let x = r.try_inverse().unwrap() * &q * &b.transpose();
    //println!("{}",x);
    let coeff = x.rows(0, variablesx);
    let intercept = x[(variablesx, 0)];
    let coe = coeff[0];
    println!(" fc = {} * edad + ({}) \n", coe, intercept);

    let y_hat = (x_test * &coeff).add_scalar(intercept);
    //println!("  y_estimada {}", y_hat.len());
    //println!("  y_prueba {}", y_test.len());
    //println!("mae: {}", mean_absolute_error(&y_test, &y_hat));
    let mut suma = 0.0;
    let mut suma2 = 0.0;
    for i in 0..y_test.len() {
        suma = suma + (y_test[i] - y_hat[i]).abs();
        suma2 = suma2 + (y_test[i] - y_hat[i]).powf(2.0);
    }
    let sumaf = suma.abs() / y_test.len() as f64;
    println!(" mae : {}", sumaf);
    let sumaf = suma2 / y_test.len() as f64;
    println!(" mse : {}", sumaf);
    let sumaf = (sumaf).powf(0.5);
    println!(" rmse: {}", sumaf);

    // ecuaciones por cintas horizontales
    for jf in csvf2 {
        let file = File::open(jf.clone()).unwrap();
        let bos: DMatrix<f64> = parse_csv(BufReader::new(file)).unwrap();
        //println!(
        //    " \n H---> CSV {} [5 renglones de muestra]: \n{}",
        //    jf,
        //    bos.rows(0, 5)
        //);
        println!(" \n H2---> CSV {}  \n", jf);
        //let nr = bos.nrows();
        let pnr = (bos.nrows() as f64 * 0.8).round() as usize;
        let nc = bos.ncols();

        //println!(" rows  {}", nr);
        //println!(" colum {}", nc);
        //println!(" %rows  {}", pnr);
        let variablesx = nc - 1;
        let x = bos.columns(0, variablesx).into_owned();
        let y = bos.column(variablesx).into_owned();

        //let (x_train, x_test, y_train, y_test) =
        //    train_test_split(&x, &y.transpose(), test_size = 0.2, shuffle = true);
        let x_train = x.rows(0, pnr as usize).into_owned();
        //println!("{}", x_train.rows(0, 5));
        //println!("  X entrenamiento \n {}", x_train.nrows());

        let x_test = x.remove_rows(0, pnr).into_owned();
        //println!("{}", x_test.rows(0, 5));
        //println!("  X de prueba \n {}", x_test.nrows());

        let y_train = y.rows(0, pnr as usize).into_owned();
        //println!("{}", y_train.rows(0, 5));
        //println!(" Y de entrenamiento \n {}", y_train.nrows());

        let y_test = y.remove_rows(0, pnr).into_owned();
        //let x_test= x.rows(pnr as usize, nr as usize).into_owned();
        //println!("{}", y_test.rows(0, 5));
        //println!("  Y de prueba \n {}", y_test.nrows());

        let a = x_train.clone().insert_column(variablesx, 1.0).into_owned();
        let b = y_train.clone().transpose();

        //-- Q, R = np.linalg.qr(A)
        // Q, R = np.linalg.qr(A)
        let qr = a.qr();
        let (q, r) = (qr.q().transpose(), qr.r());
        //let q=qr.q();
        //let r=qr.r();
        //println!("{}", r.rows(0, 5));
        let x = r.try_inverse().unwrap() * &q * &b.transpose();
        //println!("{}",x);
        let coeff = x.rows(0, variablesx);
        let intercept = x[(variablesx, 0)];
        let coe = coeff[0];
        println!(" fc = {} * edad + ({}) \n", coe, intercept);

        let y_hat = (x_test * &coeff).add_scalar(intercept);
        //println!("  y_estimada {}", y_hat.len());
        //println!("  y_prueba {}", y_test.len());
        //println!("mae: {}", mean_absolute_error(&y_test, &y_hat));
        let mut suma = 0.0;
        let mut suma2 = 0.0;
        for i in 0..y_test.len() {
            suma = suma + (y_test[i] - y_hat[i]).abs();
            suma2 = suma2 + (y_test[i] - y_hat[i]).powf(2.0);
        }
        let sumaf = suma.abs() / y_test.len() as f64;
        println!(" mae : {}", sumaf);
        let sumaf = suma2 / y_test.len() as f64;
        println!(" mse : {}", sumaf);
        let sumaf = (sumaf).powf(0.5);
        println!(" rmse: {}", sumaf);
    }
}
