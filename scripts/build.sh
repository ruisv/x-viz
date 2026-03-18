#!/usr/bin/env bash
# x-viz 本地编译脚本
#
# 用法:
#   ./scripts/build.sh                        # 编译当前平台（release）
#   ./scripts/build.sh --target aarch64-apple-darwin
#   ./scripts/build.sh --target x86_64-apple-darwin
#
# 前置要求:
#   - Rust (https://rustup.rs)
#   - npm (Node.js >= 18)
#   - @tauri-apps/cli 已在 ui/package.json devDependencies 中

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
UI_DIR="$ROOT_DIR/ui"

# ---------- 颜色输出 ----------
info()  { echo -e "\033[1;34m[INFO]\033[0m  $*"; }
ok()    { echo -e "\033[1;32m[ OK ]\033[0m  $*"; }
warn()  { echo -e "\033[1;33m[WARN]\033[0m  $*"; }
error() { echo -e "\033[1;31m[ERR ]\033[0m  $*" >&2; exit 1; }

# ---------- 解析参数 ----------
TARGET=""
while [[ $# -gt 0 ]]; do
  case $1 in
    --target) TARGET="$2"; shift 2 ;;
    *) error "未知参数: $1（支持: --target <triple>）" ;;
  esac
done

# ---------- 检查依赖 ----------
info "检查编译环境..."

command -v npm &>/dev/null || error "未找到 npm，请先安装 Node.js（https://nodejs.org）"
ok "npm: $(npm --version)"

command -v cargo &>/dev/null || error "未找到 cargo，请安装 Rust: https://rustup.rs"
ok "Rust: $(rustc --version)"

# Tauri CLI：使用项目本地 npx tauri（ui/node_modules/.bin/tauri）
TAURI_CMD="npx --prefix $UI_DIR tauri"
if $TAURI_CMD --version &>/dev/null 2>&1; then
  ok "Tauri CLI: $($TAURI_CMD --version)"
else
  error "未找到 Tauri CLI，请先在 ui/ 目录运行: npm install"
fi

# ---------- 添加 Rust 编译目标 ----------
if [[ -n "$TARGET" ]]; then
  info "添加 Rust 编译目标: $TARGET"
  rustup target add "$TARGET"
  ok "目标已就绪: $TARGET"
fi

# ---------- 安装前端依赖 ----------
info "安装前端依赖..."
cd "$UI_DIR"
npm install
ok "前端依赖安装完成"

# ---------- 执行 Tauri 编译 ----------
# tauri build 会自动执行 beforeBuildCommand（npm build in ui/），
# 因此无需在此手动编译前端。
cd "$ROOT_DIR"
info "开始编译 Tauri 应用..."

TAURI_ARGS="build"
[[ -n "$TARGET" ]] && TAURI_ARGS="$TAURI_ARGS --target $TARGET"

$TAURI_CMD $TAURI_ARGS

# ---------- 输出产物路径 ----------
RESOLVED_TARGET="${TARGET:-$(rustc -vV | grep '^host' | cut -d' ' -f2)}"
BUNDLE_DIR="$ROOT_DIR/src-tauri/target/$RESOLVED_TARGET/release/bundle"

echo ""
ok "编译成功！"
info "产物目录: $BUNDLE_DIR"
if [[ -d "$BUNDLE_DIR" ]]; then
  ls "$BUNDLE_DIR"/
fi
