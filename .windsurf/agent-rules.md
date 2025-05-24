# Agent Guidelines

## Communication Style
- Be concise and technical
- Use markdown for formatting
- Include code examples when relevant
- Reference specific files and line numbers when possible

## Code Changes
1. Always follow Rust best practices
2. Document new functions and modules
3. Write tests for new features
4. Update documentation when making changes

## Context Awareness
- Check existing files before creating new ones
- Be aware of the project's architecture
- Consider both web and desktop targets

## Decision Making
1. Propose solutions with pros/cons
2. Highlight potential risks
3. Consider performance implications
4. Check for existing implementations

## Documentation
- Keep documentation up-to-date
- Add examples for complex concepts
- Use diagrams for architectural explanations
- Document assumptions and limitations

## Error Handling
- Use proper error types
- Provide helpful error messages
- Log errors appropriately
- Consider recovery strategies

## Performance
- Be mindful of WASM binary size
- Optimize hot code paths
- Use asynchronous operations for I/O
- Profile before optimizing

## Security
- Validate all inputs
- Sanitize user-generated content
- Follow web security best practices
- Keep dependencies updated

## Testing
- Write unit tests for new features
- Add integration tests for critical paths
- Test in both web and desktop environments
- Verify cross-browser compatibility

## Code Review
- Be constructive in feedback
- Explain the reasoning behind suggestions
- Consider alternative approaches
- Check for potential edge cases
