# Agent instructions

Conventions for anyone (human or agent) working in this repository.

## Comments

- Comments that explain _what_ the code does are not welcome. The code should be self-evident; rename, extract, or restructure instead of annotating.
- Comments that explain _why_ are acceptable when the reason is non-obvious (a workaround, a constraint, a deliberate trade-off). Keep them short.
- Before writing a "why" comment, make sure it isn't a "what" comment in disguise. If it merely restates the code, delete it.

## Writing and formatting

- Use sentence-case in headings, never title-case.
- Don't use em-dashes for punctuation.
- Don't attribute work to any tool or AI in commits or code.

## Dependency injection

DI here follows Seemann & van Deursen, _Dependency Injection: Principles, Practices, and Patterns_ (Manning). Keep it clean and simple, favouring the plain patterns below over any framework or container.

- Program to abstractions, not implementations. A consumer depends on a trait (a seam), never on a concrete type. See `Host` and `Backend`.
- Traits should ideally be declared by the consumer that depends on them, not by the type that implements them. The consuming crate (or module) owns the trait and states what it needs; the implementor depends on that trait and satisfies it. This keeps the dependency arrow pointing at the consumer's abstraction and lets the implementor be swapped without the consumer ever referencing it. See `Host`, declared alongside the `Driver` that uses it.
- Constructor injection is the default. Pass dependencies in at construction, store them, and assume they are always present, with no nulls and no optional wiring. See `Driver::new(host)`.
- Keep one composition root. Concrete types are selected and wired together in a single place as close to the entry point as possible. Only the composition root names concrete implementations. See `main.rs` constructing `SystemHost` and handing it to the `Driver`.
- Only inject volatile dependencies: those that reach out of process (filesystem, network, subprocesses), carry non-determinism, or otherwise need to be substituted in tests. Don't add a seam around stable, deterministic code; just call it.
- A seam exists to be substituted. Production gets the real implementation from the composition root; tests pass a hand-written test double (a fake/stub), not a mocking framework. See `FakeHost` in `driver.rs`.

Anti-patterns to avoid:

- Control Freak: constructing a volatile dependency (`SystemHost`, an HTTP client, etc.) inside the code that uses it instead of receiving it. The only place allowed to construct them is the composition root.
- Service Locator: handing code a registry it pulls dependencies out of. Dependencies should be explicit constructor parameters, so they are visible in the type signature.
- Ambient Context / global state: reaching for a singleton or global rather than an injected dependency.
- Over-abstraction: adding a trait and injection for something stable and deterministic that no test needs to replace.
