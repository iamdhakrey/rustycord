<div align="center">
  <img src="../images/rustycord-favicon.png" alt="rustycord" width="32" height="32">
</div>

# ⚠️ Development Status

!!! danger "Not Production Ready"
    **rustycord is currently in heavy development and should NOT be used for production bots.**
    
    This library is experimental, unstable, and subject to frequent breaking changes. Use only for learning, experimentation, and development purposes.

## Current Development Phase

rustycord is in **Alpha development** with the following characteristics:

- 🚨 **Breaking changes occur frequently**
- 🚨 **APIs are unstable and will change**
- 🚨 **Features are incomplete or missing**
- 🚨 **Limited testing and validation**
- 🚨 **No backward compatibility guarantees**

## Feature Status

<table class="status-table">
<thead>
<tr>
<th>Component</th>
<th>Status</th>
<th>Notes</th>
</tr>
</thead>
<tbody>
<tr>
<td>Basic Bot Framework</td>
<td class="status-working">✅ Working</td>
<td>Core functionality implemented</td>
</tr>
<tr>
<td>HTTP Client</td>
<td class="status-working">✅ Working</td>
<td>Basic REST API calls functional</td>
</tr>
<tr>
<td>Gateway Connection</td>
<td class="status-working">✅ Working</td>
<td>WebSocket connection stable</td>
</tr>
<tr>
<td>Message Handling</td>
<td class="status-working">✅ Working</td>
<td>Message events and handlers functional</td>
</tr>
<tr>
<td>Prefix Commands</td>
<td class="status-working">✅ Working</td>
<td>Command system implemented</td>
</tr>
<tr>
<td>Logging System</td>
<td class="status-working">✅ Working</td>
<td>Comprehensive logging available</td>
</tr>
<tr>
<td>Event System</td>
<td class="status-development">⚠️ Partial</td>
<td>Basic events work, advanced features missing</td>
</tr>
<tr>
<td>Embed Support</td>
<td class="status-development">⚠️ Partial</td>
<td>Basic embeds work, complex features missing</td>
</tr>
<tr>
<td>Slash Commands</td>
<td class="status-missing">❌ Missing</td>
<td>Not implemented yet</td>
</tr>
<tr>
<td>Voice Support</td>
<td class="status-missing">❌ Missing</td>
<td>Not planned for initial release</td>
</tr>
<tr>
<td>Sharding</td>
<td class="status-missing">❌ Missing</td>
<td>Basic structure exists, not functional</td>
</tr>
<tr>
<td>Rate Limiting</td>
<td class="status-missing">❌ Missing</td>
<td>No rate limit handling</td>
</tr>
<tr>
<td>Comprehensive Testing</td>
<td class="status-missing">❌ Missing</td>
<td>Minimal test coverage</td>
</tr>
<tr>
<td>Documentation</td>
<td class="status-development">⚠️ Partial</td>
<td>Basic docs exist, many gaps</td>
</tr>
<tr>
<td>Stable API</td>
<td class="status-missing">❌ Missing</td>
<td>APIs change frequently</td>
</tr>
</tbody>
</table>

## What Works Now

You can use rustycord for:

- ✅ **Learning Discord bot development**
- ✅ **Experimenting with Rust async programming**
- ✅ **Building simple test bots**
- ✅ **Contributing to development**
- ✅ **Prototyping bot ideas**

## What Doesn't Work

Do not expect:

- ❌ **Production-ready stability**
- ❌ **Complete Discord API coverage**
- ❌ **Backward compatibility**
- ❌ **Professional support**
- ❌ **Performance optimization**
- ❌ **Advanced Discord features**

## Development Roadmap

### Phase 1: Core Stability (Current)
- Fix remaining message handling issues
- Improve error handling
- Add basic testing
- Stabilize core APIs

### Phase 2: Essential Features
- Implement slash commands
- Add rate limiting
- Improve sharding support
- Enhanced embed system

### Phase 3: Advanced Features
- Voice support consideration
- Performance optimization
- Comprehensive testing
- API documentation

### Phase 4: Stability & Release
- API freeze for 1.0
- Comprehensive testing
- Production readiness
- Stable release

## Risk Assessment

| Risk Level | Description |
|------------|-------------|
| 🔴 **HIGH** | Your bot may break with library updates |
| 🔴 **HIGH** | Data loss or corruption possible |
| 🔴 **HIGH** | Security vulnerabilities may exist |
| 🟡 **MEDIUM** | Performance issues expected |
| 🟡 **MEDIUM** | Limited feature set |
| 🟢 **LOW** | Good for learning and experimentation |

## Alternatives for Production

If you need a production-ready Discord bot library for Rust, consider:

- **[Serenity](https://github.com/serenity-rs/serenity)** - Mature, stable, production-ready
- **[Twilight](https://github.com/twilight-rs/twilight)** - Modern, modular, well-maintained
- **[Poise](https://github.com/serenity-rs/poise)** - Command framework built on Serenity

## Contributing

Despite being in development, contributions are welcome:

- 🐛 **Bug reports** - Help identify issues
- 💡 **Feature requests** - Suggest improvements
- 🔧 **Code contributions** - Submit pull requests
- 📚 **Documentation** - Improve guides and examples
- 🧪 **Testing** - Help test new features

See the [GitHub repository](https://github.com/iamdhakrey/rustycord) for contribution guidelines.

## Getting Updates

Stay informed about rustycord development:

- ⭐ **Star the repository** for notifications
- 👀 **Watch releases** for version updates
- 📋 **Check issues** for known problems
- 💬 **Join discussions** for community updates

## Legal Notice

By using rustycord, you acknowledge:

- This software is provided "as is" without warranty
- The developers are not responsible for any issues or damages
- Use in production environments is strongly discouraged
- Breaking changes may occur at any time

---

**Remember: Wait for the stable 1.0 release before considering production use.**
