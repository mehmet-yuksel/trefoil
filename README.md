# Trefoil: An AST-Based Version Control Experiment

Trefoil is a simple, version control system (VCS) written in Rust. Unlike traditional VCS like Git which primarily operate on lines of text, Trefoil works directly with the **Abstract Syntax Tree (AST)** of the code. It also stores changes as **structural diffs** (instructions to transform one AST into another) rather than full file snapshots.

## Motivation: Why Bother?

Honestly? Mostly because it sounded cool.

1.  **The "Git Stores Copies?" Moment:** I remember learning about how Git stores data – primarily as snapshots of files. While incredibly effective and optimized, the idea that it wasn't purely storing *diffs* felt... counter-intuitive at first? It sparked a thought: "What would a VCS look like if it *only* stored the precise changes needed to get from one version to the next?" *(Disclaimer: I know Git is way more complex and efficient than just 'storing copies', but this was the initial naive spark!)*

2.  **The Allure of Structural Editing:** Around the same time, I came across the concept of structural code editors (like Paredit for Lisp, or experimental AST-based editors). The idea of manipulating code based on its *structure* rather than just text characters seemed really interesting. It felt like a more fundamentally "correct" way to interact with code in some sense.

3.  **Putting Them Together:** Trefoil is basically my attempt to smash these two ideas together. Could I build a VCS that understands code structure (via ASTs) and stores history as structural transformations? This project is the result of that exploration. Again, it's not meant to be practical, just a fun way to learn about parsing, diffing algorithms, and AST manipulation.

## How It Works

Trefoil is designed for a simple Lisp-like syntax (think `(operator operand1 operand2)`). Here's the basic workflow:

1.  **Parsing:** When you commit `code.lisp`, Trefoil **parses** the file's text content into an Abstract Syntax Tree (AST). The parser reads the entire file, and internally represents the sequence of top-level forms (like multiple definitions or expressions) as a root `Ast::List` node. (`src/parser.rs`, `src/ast.rs`)
2.  **Diffing:** It then loads the AST of the *previous* commit. Trefoil **compares** the old AST with the new AST to find the structural differences. (`src/diff.rs`)
3.  **Instructions:** The difference is captured as a list of specific **structural instructions**, operating on paths within the AST structure, like:
    *   `Update the atom at path [0, 1] to "y"` (e.g., update the second element within the first top-level form)
    *   `Insert the node List(...) at path [] index 2` (e.g., insert a new top-level form)
    *   `Delete the node at path [] index 0` (e.g., delete the first top-level form)
    *   `Replace the node at path [1] with Atom("new")` (e.g., replace the entire second top-level form)
    *(The `path` originates from the conceptual root)*. (`src/instruction.rs`)
4.  **Storing Commits:** A new "commit" object is created containing *only* these instructions, a unique ID, and the parent commit's ID. It doesn't store a full copy of the code. Commits are saved as JSON files. (`src/vc.rs`)
5.  **Reconstruction & Checkout:** To check out a specific version, Trefoil starts from the initial empty state (commit 0) and **applies** the stored instruction sequences from commits 1 up to the target commit ID. This reconstructs the AST for that version.
    When writing this AST back to `code.lisp`, the `checkout` command formats it appropriately: if the reconstructed AST represents a sequence of top-level forms (internally an `Ast::List`), it converts each form back to its string representation and joins them with **newlines**. This ensures the output file looks like the original Lisp code structure. (`src/apply.rs`, `src/vc.rs::reconstruct_ast`, `src/main.rs::checkout`)

Essentially, the repository stores a history of structural transformations, allowing the reconstruction of any version's AST, which is then formatted correctly back into a file.

## Usage Examples

First, create a directory for your project and `cd` into it.

1.  **Initialize:** Set up the repository.
    ```bash
    cargo run -- init
    # Creates .trefoil/ and the initial commit 0. Creates 'code.lisp' if needed.
    # Output: Initialized empty repository...
    ```

2.  **Make Changes:** Edit `code.lisp`.
    ```lisp
    # code.lisp
    (define x 10)
    (print x)
    ```

3.  **Commit:** Save your changes.
    ```bash
    cargo run -- commit
    # Parses code.lisp, diffs against commit 0, saves instructions as commit 1.
    # Output: Committed changes as commit 1
    ```

4.  **Make More Changes:**
    ```lisp
    # code.lisp
    (define y 20)
    (print y)
    (display "done")
    ```

5.  **Commit Again:**
    ```bash
    cargo run -- commit
    # Diffs commit 1's state against the new state, saves instructions as commit 2.
    # Output: Committed changes as commit 2
    ```
    *(If no changes were made, it reports: "No changes detected...")*

6.  **View History:**
    ```bash
    cargo run -- log
    # Output:
    # Commit History (newest first):
    # * commit 2 (parent: Some(1)) (HEAD)
    # * commit 1 (parent: Some(0))
    # * commit 0 (parent: None)
    ```

7.  **Checkout a Previous Version:** Restore `code.lisp` to commit 1's state.
    ```bash
    cargo run -- checkout 1
    # Reconstructs commit 1's AST and writes it correctly formatted to code.lisp.
    # Updates HEAD.
    # Output: Checked out commit 1. 'code.lisp' updated.
    ```
    `code.lisp` will now contain:
    ```lisp
    (define x 10)
    (print x)
    ```

8.  **Debug Instructions:** See the changes stored *in* commit 2.
    ```bash
    cargo run -- debug 2
    # Shows the instructions transforming state from commit 1 to commit 2.
    # Output might look like:
    # Instructions stored IN commit 2: (Transforming from parent Some(1) to 2)
    # 1. Update at path (0 1) with value y
    # 2. Update at path (0 2) with value 20
    # 3. Update at path (1 1) with value y
    # 4. Insert at path () index 2 node (display "done")
    ```

## Limitations (Still plenty!)

*   **Naive Diffing:** The diff algorithm is basic. Changes involving different list lengths often result in replacing the whole list rather than minimal inserts/deletes. A proper tree diff algorithm would be much better.
*   **Basic Syntax Only:** Handles simple S-expressions (atoms matching `[a-zA-Z0-9_]+` and lists).
*   **No Branching/Merging:** Linear history only.
*   **Performance:** Reconstruction involves replaying all instructions from the beginning, which will be slow for long histories.
*   **AST -> String Formatting:** While `checkout` produces structurally correct output, the exact original whitespace/indentation is lost.
