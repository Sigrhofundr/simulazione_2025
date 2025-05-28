<h1>Esercitazione 7 Rust - Malnati 28 Maggio 2025</h1>
<h2>Esercizio 1: Simulazione d'esame </h2>

> [!NOTE]
> Questo esercizio è da risolvere con l’istanza di Visual Studio Code che si può far partire dal proprio profilo di Moodle.
><ul>
><li>Visualizzare la domanda con il testo del problema e premere su “Apri o accedi a CrownLabs”</li>
><li>VSCode si aprirà in un nuovo tab del browser con un progetto Rust vuoto, in cui svolgere l’esercizio</li>
><li>Una volta aperta la finestra di VSCode nel browser si potrà sospendere, chiudere e riaprire.</li>
><li>Finito l’esercizio premere su “termina il tentativo” e poi su “Invia e termina”</li>
></ul>
>Per comodità riportiamo il testo anche qui di seguito.

Un countdown latch è un oggetto che permette di sincronizzare thread con le seguenti caratteristiche:
<ul>
<li>L’oggetto contiene un contatore che viene inizializzato con valore N</li>
<li>Il latch può essere condiviso su più thread</li>
<li>Quando un thread chiama il metodo wait_zero() del latch si blocca finché il contatore non va a zero</li>
<li>Quando viene chiamato count_down() il contatore viene decrementato di 1</li>
<li>Quando il contatore va a zero tutti i thread bloccati su wait_zero() vengono sbloccati</li>
</ul>
Implementare la struct CountDownLatch con la seguente interfaccia:

```
impl CountDownLatch {
    pub fn new(n: usize) -> Self {}
    // wait zero aspetta al massimo timeout ms
    // se esce per timeout ritorna Err altrimenti Ok
    pub fn wait_zero(&self, timeout: Option<std::time::Duration>) -> Result<(),()>
    pub fn count_down(&self) {}
}
```

Implementato il CountDownLatch, utilizzarlo per completare l’esempio seguente, in cui dei
thread hanno bisogno di un driver per eseguire del lavoro; il driver viene preparato e
rilasciato dal thread principale:
<li>i thread non possono eseguire (2) finché il driver non è pronto</li>
<li>il driver è pronto dopo (1)</li>
<li>il driver deve essere rilasciato appena i thread non ne hanno più bisogno, quindi (4)
deve essere chiamato il prima possibile dopo (2) senza rallentare (3) e senza
aspettare la fine dei thread</li>
<li>doSomeWork() è una funzione che simula lavoro con una sleep()</li>

```
pub fn demo_latch() {
    let mut handles = vec![];
    for _ in 0..10 {
        let h = thread::spawn(||{
            doSomeWork("(2) lavoro che necessita driver");
            doSomeWork("(3) altro lavoro che non necessita driver");
        });
        handles.push(h);
    } 
    doSomeWork("(1) prepapara il driver");
    doSomeWork("(4) rilascia il driver");
    for h in handles {
        let _ = h.join();
    }
}
```
