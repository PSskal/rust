struct FuncionTransferencia {
    ganancia: f64,
    constante_tiempo: f64,
}

impl FuncionTransferencia {
    fn new(ganancia: f64, constante_tiempo: f64) -> Self {
        FuncionTransferencia {
            ganancia,
            constante_tiempo,
        }
    }

    fn respuesta(&self, entrada: f64, tiempo: f64) -> f64 {
        let denominador = self.constante_tiempo * tiempo + 1.0;
        let salida = self.ganancia * (1.0 - (-tiempo / self.constante_tiempo).exp()) * entrada / denominador;
        salida
    }
}

fn main() {
    let funcion_transferencia = FuncionTransferencia::new(2.0, 1.0);

    // Crear un vector de tiempo de 0 a 5 segundos con incrementos de 0.1 segundos
    let tiempo: Vec<f64> = (0..=50).map(|i| i as f64 * 0.1).collect();

    // Crear un vector para almacenar la respuesta de la función de transferencia en cada instante de tiempo
    let mut respuesta: Vec<f64> = Vec::new();

    let entrada = 1.0;

    // Calcular la respuesta de la función de transferencia para cada instante de tiempo
    for t in &tiempo {
        let r = funcion_transferencia.respuesta(entrada, *t);
        respuesta.push(r);
    }

    // Imprimir la respuesta en cada instante de tiempo
    for (t, r) in tiempo.iter().zip(respuesta.iter()) {
        println!("Tiempo: {:.1} s, Respuesta: {:.4}", t, r);
    }
}
