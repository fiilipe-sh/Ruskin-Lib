![banner](https://github.com/user-attachments/assets/a2010bd4-613f-4d16-b03e-c73ff86dbac0)<p align="center">
</p>

<br/>

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.70%2B-e8622a?style=flat-square&logo=rust&logoColor=white"/>
  <img src="https://img.shields.io/badge/license-MIT-a8c97f?style=flat-square"/>
  <img src="https://img.shields.io/badge/crates.io-v0.1.0-6a9fb5?style=flat-square"/>
</p>

<br/>

**ruskin** é uma biblioteca Rust para leitura de input no terminal — simples, sem dependências, com retry automático e validação embutida.

---

## instalação

Adicione ao seu `Cargo.toml`:

```toml
[dependencies]
ruskin = "0.1"
```

---

## uso

```rust
use ruskin::*;

fn main() {
    let nome  = prompt_string("Seu nome:");
    let idade = prompt_int("Sua idade:");
    let peso  = prompt_f64("Seu peso (kg):");
    let admin = prompt_bool("É admin? (s/n):");

    let plano = prompt_menu("Escolha seu plano:", &["Gratuito", "Pro", "Enterprise"]);

    let email = prompt_validated(
        "Seu e-mail:",
        |s| if s.contains('@') { Some(s.to_string()) } else { None },
        "e-mail deve conter '@'.",
    );
}
```

---

## funções

| função | retorno | descrição |
|---|---|---|
| `prompt_string(prompt)` | `String` | lê texto não-vazio |
| `prompt_string_optional(prompt)` | `Option<String>` | lê texto, aceita vazio |
| `prompt_int(prompt)` | `i64` | lê inteiro |
| `prompt_f64(prompt)` | `f64` | lê decimal |
| `prompt_u64(prompt)` | `u64` | lê inteiro sem sinal |
| `prompt_usize(prompt)` | `usize` | lê usize |
| `prompt_number::<T>(prompt)` | `T` | lê qualquer tipo numérico |
| `prompt_number_range(prompt, min, max)` | `T` | número dentro de um intervalo |
| `prompt_bool(prompt)` | `bool` | lê s/n (e variações) |
| `prompt_menu(prompt, &[options])` | `usize` | menu numerado, retorna índice |
| `prompt_validated(prompt, fn, err)` | `T` | validação por closure |
| `prompt_confirm(message)` | `bool` | confirmação de ação destrutiva |

Todas as funções repetem automaticamente até receber uma entrada válida.

---

## exemplos

**número com intervalo**
```rust
let nota = prompt_number_range("Nota (0–10):", 0.0_f64, 10.0);
```

**tipo genérico**
```rust
let x: i32 = prompt_number("Digite um i32:");
```

**validação customizada**
```rust
let cpf = prompt_validated(
    "CPF (somente números):",
    |s| {
        let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() == 11 { Some(digits) } else { None }
    },
    "CPF deve ter 11 dígitos.",
);
```

**confirmação antes de ação destrutiva**
```rust
if prompt_confirm("Isso vai apagar todos os dados.") {
    deletar_tudo();
}
```

---

## comportamento de erro

Nenhuma função entra em pânico com input inválido — todas exibem uma mensagem de erro e pedem a entrada novamente. Para capturar erros programaticamente, use `prompt_validated` com sua própria lógica de retorno.

---

## licença

MIT — veja [LICENSE](LICENSE).
