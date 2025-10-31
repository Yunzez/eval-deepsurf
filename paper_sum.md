# deepSURF: Detecting Memory Safety Vulnerabilities in Rust Through Fuzzing LLM-Augmented Harnesses

## Problem Space
- Rust’s safety guarantees break down inside `unsafe` functions/blocks and unsafe trait implementations, where developers must manually uphold invariants.
- Existing Rust-focused bug-finding tools either rely on static pattern matching (leading to high false-positive rates), lack support for Rust-specific abstractions (traits, generics, closures), or cannot automatically generate realistic fuzz harnesses.
- Fuzzing excels at surfacing memory corruption, but most Rust crates are libraries that require bespoke harnesses to expose unsafe call chains reachable through safe APIs (URAPIs).

## deepSURF Overview
deepSURF combines static analysis with LLM-guided augmentation to automatically produce compilable fuzz harnesses that target unsafe code paths within Rust libraries. The system focuses on identifying **Unsafe Reaching APIs (URAPIs)**—public functions that call or wrap unsafe code—and exercising them with semantically related API sequences that mimic real-world usage.

### Pipeline
1. **Static Analysis (modified `rustc`)**
   - Builds control-flow graphs for each crate, classifies functions into unsafe categories (unsafe functions, unsafe encapsulating functions, unsafe reaching functions), and extracts URAPIs.
   - Performs *dependency tree construction* to determine feasible argument instantiations, including recursive resolution of nested types.
   - Synthesizes an initial harness per URAPI, selecting constructors for complex arguments and recording metadata (type info, trait bounds, discovered URAPIs, doc snippets).
2. **Harness Selector**
   - Filters static harnesses to retain compilable, high-value candidates.
   - Prioritizes harnesses that include custom user-defined behaviors (e.g., trait implementations or closures) because they often expose bugs.
3. **LLM-Based Harness Augmenter**
   - Feeds a prompt containing the static harness, URAPI metadata, and augmentation guidelines to the default DeepSeek-R1 backend.
   - Requests the model to enrich harnesses with additional API calls, stateful workflows, custom types, and closure implementations while keeping the target URAPI reachable.
   - Applies heuristics to skip redundant augmentations (e.g., when a URAPI is already covered by an earlier harness unless custom logic is required).
4. **Fuzzing Corpus Assembly**
   - Collects augmented harnesses and any necessary fallbacks from the static stage.
   - Compiles each harness inside an afl.rs-based driver configured with ASan, CmpLog, persistent mode, and logic to ignore non-memory panics.
5. **Fuzzing Execution**
   - Runs two AFL++ workers per harness (ASan + CmpLog) for 24 hours.
   - Classifies crashes, focusing on memory corruption (double-free, buffer overflows, use-after-free, arbitrary memory writes).

### Notable Innovations
- **Generics & Traits:** Automatically substitutes generic parameters with synthesized types and generates trait implementations, including unsafe traits that require manually enforced invariants.
- **Closures & Callbacks:** Generates closure bodies that simulate adversarial user code capable of provoking unsafe behavior.
- **Search Space Guidance:** Uses LLM reasoning to prioritize API sequences likely to expose bugs and to decide between multiple candidate argument instantiations.
- **False-Positive Mitigation:** Targets only safe URAPIs (rather than raw unsafe blocks) and filters pure panic crashes to minimize noise.

## Evaluation Highlights
- **Dataset:** 63 real-world crates drawn from ERASAN, RUSTSAN, RUG, and CrabTree corpora.
- **Bug Findings:** 42 memory corruption bugs uncovered (30 previously known, 12 new). Eleven new bugs have RustSec advisories; three were patched after disclosure.
- **Coverage:** Achieved 87.3 % URAPI coverage across the dataset, far exceeding RUG (22.5 %), RPG (4.1 %), and RULF (3 %).
- **Comparative Results:** In the ERASAN subset (27 crates), deepSURF detected 26 bugs (6 new). Competing tools either failed to find memory bugs or crashed/timed out on several crates.
- **Ablation Study:**
  - Removing LLM augmentation (`deepSURF-static`) reduced bug findings to 3 and lowered unsafe coverage, highlighting the necessity of enriched API sequences.
  - Removing static analysis (`deepSURF-llm`) hindered URAPI discovery and trait handling, missing complex bugs despite reasonable coverage in simpler crates.
  - Harness selection policies matter: the default skip strategy (keep statically generated harnesses with custom behavior) found more bugs than “augment everything” or “skip any duplicates.”
  - Alternative LLM backends (OpenAI o3, Claude Sonnet 4, GPT-5) delivered comparable coverage but at higher cost or lower reliability; DeepSeek-R1 balanced cost and success rate best.
- **Case Studies:** Detailed analyses of slice-deque bugs show deepSURF constructing harnesses that combine constructors, mutating APIs, and user-defined closures to trigger double-free vulnerabilities.

## Limitations & Future Work
- Missed 26 known bugs, often requiring capabilities outside the current scope (async, multithreading, FFI, or interaction with `std::mem::forget`).
- Harness augmentation can still fail when prompts exceed context limits or when LLM hallucinations introduce invalid code; retries mitigate but do not eliminate this risk.
- Arithmetic overflow panics are filtered by default, so deepSURF may miss bugs that manifest only when overflow checks are disabled (release builds).
- Future directions include richer qualitative assessments of harness realism, expanding support for additional Rust features, and integrating smarter triaging for non-memory crashes.

## Key Takeaways
- Targeting URAPIs and combining compiler-guided scaffolding with LLM creativity enables high-quality harness generation.
- Support for generics, custom traits, and closures is critical to fuzzing unsafe Rust effectively.
- deepSURF demonstrates that LLM-augmented fuzzing can rediscover known vulnerabilities and uncover new ones in mature crates with limited manual effort.
