# 实验室极简记账 · Lab Ledger

> Tauri + Vue3 + Naive UI + Rust + SQLite · 极简实验室财务流水

## 特性
- 极简流水：日期 / 收支 / 金别 / 金额 / 说明 + 多图附件
- 实时余额：零起累计，列表每行显示结余
- 本地存储：SQLite 单文件 `appData/lab-ledger.db`，图片存 `appData/images/YYYY-MM/`
- 类别可自定义：预置 耗材/设备/差旅/劳务/测试费/其他 等
- 检索统计：关键词 / 收支 / 类别 / 日期区间 + 按类别/按月汇总
- 导出 Excel：按当前筛选导出到下载目录

## 目录
```
lab-ledger/
  src/            # Vue3 前端
  src-tauri/      # Rust 后端 + SQLite
```

## 快速开始

```bash
# 1. 安装依赖
npm install

# 2. 开发运行（需 Rust 已安装）
npm run tauri dev
# 或仅前端预览（无后端，走 Mock 数据）
npm run dev
```

```bash
# 打包 Windows 安装包
npm run tauri build
# 产物在 src-tauri/target/release/bundle/
```

## 数据库
- `categories` / `transactions` / `transaction_images` / `settings`
- 金额以分为单位存储，前端以元展示
- 删除流水会级联删除图片文件；被使用的类别禁止删除

## 说明
- 单机单人，无登录、无联网
- 仅 Windows 打包已验证，macOS/Linux 理论可用
- 图片通过 base64 传入 Rust 落盘，前端预览用 `convertFileSrc` 读取本地文件

