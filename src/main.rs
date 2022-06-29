fn main(){
    //obtener la pildora desde la consola  consola
    println!("Esta es tu tultima oportunidad, despues de esto no hay vuelta atras. \
    tomas la pildora azul todo esto termina te despiertas en tu cama y crees  \
    lo que quieras creer. Tomas la pildora roja: Te quedas en el lugar donde \
    solo estamos los despiertos \
    y te enseño lo profundo que es la madriguera del conejo, recuerda todo lo que ofrezco es\
    la verdad. Nada más. \
    elige : \n 1. Tomar la pildora azul \n 2. Tomar la pildora roja");

    let mut pildora : String = String::new();
    std::io::stdin().read_line(&mut pildora).unwrap();

    //Limpiar entrada de pildora
     pildora = pildora.trim().to_string();

     //Si es roja nos quedamos en la Matrix
     //Si es azul, salimos de la Matrix
     //Cualquier otra opción la Matrix explota

     if pildora == "roja"{
        println!("Bienvenido, llego el momento de seguir al conejo blanco hasta su madriguera");
     } else if pildora =="azul"{
        println!("Vuelves a tu vida normal, no pasa nada...");
     } else { 
        println!("La Matrix explota");
     }


// Language: rust


    }