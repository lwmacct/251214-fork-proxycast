---
title: ProxyCast - 把你的 AI 客户端额度用到任何地方
description: 一款基于 Tauri 的桌面应用，将 Kiro、Gemini CLI、Qwen 等 AI 客户端凭证转换为标准 OpenAI/Claude 兼容 API
navigation: false
---

<div class="max-w-4xl mx-auto">

<div class="text-center py-8">
  <h1 class="text-5xl font-bold text-primary-600 mb-4">ProxyCast</h1>
  <p class="text-3xl font-semibold text-gray-700 dark:text-gray-300 mb-2">把你的 AI 客户端额度用到任何地方</p>
  <p class="text-xl text-gray-600 dark:text-gray-400 mb-4">一款基于 Tauri 的桌面应用，将 Kiro、Gemini CLI、Qwen 等 AI 客户端凭证转换为标准 OpenAI/Claude 兼容 API</p>
  <p class="text-base text-gray-500 dark:text-gray-500 mb-10">凭证池管理 • 智能路由 • 协议转换 • 容错机制</p>
  <div class="flex gap-4 justify-center flex-wrap">
    <a href="/introduction/quickstart" class="inline-block px-8 py-3 bg-primary-600 text-white font-medium rounded-lg hover:bg-primary-700 transition-colors">快速开始</a>
    <a href="https://github.com/aiclientproxy/proxycast" target="_blank" class="inline-block px-8 py-3 border-2 border-gray-300 dark:border-gray-600 font-medium rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors">GitHub</a>
  </div>
</div>

<div class="text-center py-3 px-4 mb-6 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg">
  <p class="text-yellow-800 dark:text-yellow-200">
    <strong>⚠️ 免责声明:</strong> 本工具仅限于个人合法使用，严禁用于非法盈利目的。初衷是帮助用户充分利用已订阅的 AI 服务 Token。
    <a href="/legal/disclaimer" class="text-primary-600 hover:underline ml-1">查看完整声明</a>
  </p>
</div>

## ✨ 核心特性

<div class="grid grid-cols-1 md:grid-cols-2 gap-6 my-8">
  <div class="p-6 border border-gray-200 dark:border-gray-700 rounded-lg">
    <h3 class="text-xl font-semibold mb-3">🔑 凭证池管理</h3>
    <p class="text-gray-600 dark:text-gray-400">支持多种 AI 客户端凭证的统一管理，包括 Kiro、Gemini CLI、Qwen、Claude Code 等，自动检测和刷新 OAuth Token。</p>
  </div>
  <div class="p-6 border border-gray-200 dark:border-gray-700 rounded-lg">
    <h3 class="text-xl font-semibold mb-3">🔀 智能路由</h3>
    <p class="text-gray-600 dark:text-gray-400">基于模型名称的请求路由，支持负载均衡、优先级配置、健康检查和自动故障转移。</p>
  </div>
  <div class="p-6 border border-gray-200 dark:border-gray-700 rounded-lg">
    <h3 class="text-xl font-semibold mb-3">🛡️ 容错配置</h3>
    <p class="text-gray-600 dark:text-gray-400">内置熔断器、重试机制、超时控制，确保服务稳定性，优雅处理 API 故障。</p>
  </div>
  <div class="p-6 border border-gray-200 dark:border-gray-700 rounded-lg">
    <h3 class="text-xl font-semibold mb-3">⚡ 配置切换</h3>
    <p class="text-gray-600 dark:text-gray-400">一键切换 Claude Code、Codex、Gemini CLI 等客户端配置，快速适应不同使用场景。</p>
  </div>
  <div class="p-6 border border-gray-200 dark:border-gray-700 rounded-lg">
    <h3 class="text-xl font-semibold mb-3">📊 监控统计</h3>
    <p class="text-gray-600 dark:text-gray-400">实时监控请求统计、Token 使用追踪、详细的请求日志和性能指标。</p>
  </div>
  <div class="p-6 border border-gray-200 dark:border-gray-700 rounded-lg">
    <h3 class="text-xl font-semibold mb-3">🔌 API 兼容</h3>
    <p class="text-gray-600 dark:text-gray-400">完整支持 OpenAI Chat Completions API 和 Claude Messages API，无缝集成现有工具。</p>
  </div>
</div>

## 🎯 支持的 Provider

| Provider | 类型 | 认证方式 | 说明 |
|----------|------|----------|------|
| Kiro Claude | OAuth | 自动刷新 | AWS Kiro IDE 的 Claude 凭证 |
| Gemini CLI | OAuth | 自动刷新 | Google Gemini CLI 凭证 |
| Qwen (通义千问) | OAuth | 自动刷新 | 阿里云通义千问凭证 |
| OpenAI Custom | API Key | 手动配置 | 自定义 OpenAI 兼容服务 |
| Claude Custom | API Key | 手动配置 | 自定义 Claude 兼容服务 |

## 🚀 快速开始

### 1. 下载安装

从 [GitHub Tags](https://github.com/aiclientproxy/proxycast/tags) 下载适合你系统的安装包。

### 2. 加载凭证

ProxyCast 会自动检测本地的 AI 客户端凭证文件：

```
~/.kiro/credentials.json          # Kiro Claude
~/.config/gemini-cli/oauth_creds.json  # Gemini CLI
~/.config/qwen/credentials.json   # Qwen
```

### 3. 启动服务

点击仪表盘的「启动服务」按钮，API Server 默认运行在 `http://127.0.0.1:8999`。

### 4. 测试 API

```bash
curl http://127.0.0.1:8999/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-api-key" \
  -d '{
    "model": "claude-sonnet-4-20250514",
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```

## 📖 文档导航

<div class="grid grid-cols-1 md:grid-cols-3 gap-4 my-8">
  <a href="/introduction/overview" class="block p-4 border border-gray-200 dark:border-gray-700 rounded-lg hover:border-primary-500 transition-colors">
    <h3 class="font-semibold mb-2">📋 概述</h3>
    <p class="text-sm text-gray-600 dark:text-gray-400">了解 ProxyCast 的核心功能和价值</p>
  </a>
  <a href="/introduction/installation" class="block p-4 border border-gray-200 dark:border-gray-700 rounded-lg hover:border-primary-500 transition-colors">
    <h3 class="font-semibold mb-2">📥 安装指南</h3>
    <p class="text-sm text-gray-600 dark:text-gray-400">下载并安装 ProxyCast</p>
  </a>
  <a href="/introduction/quickstart" class="block p-4 border border-gray-200 dark:border-gray-700 rounded-lg hover:border-primary-500 transition-colors">
    <h3 class="font-semibold mb-2">🚀 快速开始</h3>
    <p class="text-sm text-gray-600 dark:text-gray-400">5 分钟内完成首次 API 调用</p>
  </a>
  <a href="/providers/overview" class="block p-4 border border-gray-200 dark:border-gray-700 rounded-lg hover:border-primary-500 transition-colors">
    <h3 class="font-semibold mb-2">🔧 Provider 配置</h3>
    <p class="text-sm text-gray-600 dark:text-gray-400">配置各种 AI 服务提供商</p>
  </a>
  <a href="/api-reference/overview" class="block p-4 border border-gray-200 dark:border-gray-700 rounded-lg hover:border-primary-500 transition-colors">
    <h3 class="font-semibold mb-2">📚 API 参考</h3>
    <p class="text-sm text-gray-600 dark:text-gray-400">完整的 API 端点文档</p>
  </a>
  <a href="/troubleshooting/common-issues" class="block p-4 border border-gray-200 dark:border-gray-700 rounded-lg hover:border-primary-500 transition-colors">
    <h3 class="font-semibold mb-2">🔍 故障排除</h3>
    <p class="text-sm text-gray-600 dark:text-gray-400">常见问题和解决方案</p>
  </a>
</div>

## 🌐 开放平台

<div class="grid grid-cols-1 md:grid-cols-2 gap-4 my-8">
  <a href="/open-platform/overview" class="block p-4 border border-gray-200 dark:border-gray-700 rounded-lg hover:border-primary-500 transition-colors">
    <h3 class="font-semibold mb-2">🔌 插件系统</h3>
    <p class="text-sm text-gray-600 dark:text-gray-400">通过插件扩展 ProxyCast 功能，支持工具类、Hook 类等多种插件类型</p>
  </a>
  <a href="/open-platform/connect" class="block p-4 border border-gray-200 dark:border-gray-700 rounded-lg hover:border-primary-500 transition-colors">
    <h3 class="font-semibold mb-2">🔗 ProxyCast Connect</h3>
    <p class="text-sm text-gray-600 dark:text-gray-400">中转商生态合作方案，一键配置 API Key，提升用户转化率</p>
  </a>
</div>

## 🤝 社区与支持

- **GitHub Issues**: [报告问题](https://github.com/aiclientproxy/proxycast/issues)
- **GitHub Discussions**: [参与讨论](https://github.com/aiclientproxy/proxycast/discussions)

## 📄 开源协议

ProxyCast 采用 [MIT License](https://github.com/aiclientproxy/proxycast/blob/main/LICENSE) 开源。

</div>
