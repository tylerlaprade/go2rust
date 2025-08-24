# Next Steps

Looking at the XFAIL tests, the major features that
need implementation include:

## Anonymous Structs (Partially Complete)

✅ Composite literals with nil Type inference
✅ Nested field access with proper unwrapping
• Basic anonymous structs (anonymous_structs_basic)
• In functions (anonymous_structs_functions)
• Nested anonymous structs
(anonymous_structs_nested)

## Const Improvements

• Complex const expressions (const_expressions)
• Iota in const blocks (const_iota)
• String concatenation in consts
(const_string_concat)
• Typed constants (const_typed)

## Import System

• Package aliases (import_aliases)
• Blank imports with side effects
(blank_imports_side_effects)
• Multiple file packages (package_multiple_files)

## Concurrency

• Goroutines (goroutines_basic, goroutines_simple)
• Sync primitives (sync_mutex, sync_waitgroup)
• Atomic operations (atomic_operations)
• Concurrency patterns (concurrency_patterns)

## Channel Operations (multiple tests)

• Basic channels (channels_basic, channels_simple)
• Buffered channels (channel_buffering)
• Channel directions (channel_directions)
• Channel synchronization (channel_sync)
• Closing channels (closing_channels)
• Select statements (select_basic,
select_statements)

## Advanced Control Flow

• Goto and labels (goto_labels, labeled_statements)
• Fallthrough in switch (fallthrough_switch)
• Complex control flow (advanced_control_flow)

## Other Features

• Variadic functions (variadic_functions)
• Named return values (named_returns)
• Init functions (init_functions,
init_order_complex)
• Panic and recover (panic_recover)
• Type switches (type_switches)
• Method value expressions (method_values)
