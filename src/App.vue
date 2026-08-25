<template>
  <n-config-provider :theme-overrides="themeOverrides">
    <n-message-provider>
      <n-dialog-provider>
        <div class="app-layout">
          <header class="topbar">
            <div class="brand">
              <div class="logo">◈</div>
              <div>
                <div class="title">实验室记账</div>
                <div class="subtitle">极简 · 本地 · 安全</div>
              </div>
            </div>
            <nav class="nav">
              <router-link to="/" :class="{active: route.path==='/'}">流水</router-link>
              <router-link to="/stats" :class="{active: route.path==='/stats'}">统计</router-link>
              <router-link to="/settings" :class="{active: route.path==='/settings'}">设置</router-link>
            </nav>
          </header>
          <main class="main">
            <router-view />
          </main>
        </div>
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { useRoute } from "vue-router";
import { NConfigProvider, NMessageProvider, NDialogProvider } from "naive-ui";
const route = useRoute();
const themeOverrides = {
  common: { primaryColor: "#2B5CFF", primaryColorHover: "#4A77FF" },
  Card: { paddingMedium: "16px 18px" }
};
</script>

<style>
* { box-sizing: border-box; }
html, body {
  margin: 0;
  height: 100%;
  overflow: hidden;
  font-family: ui-sans-serif, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial;
  background: #F5F6F8;
  color: #1A1E2B;
}
#app {
  height: 100vh;
  height: 100dvh;
  overflow: hidden;
}
.app-layout {
  height: 100vh;
  height: 100dvh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: #F5F6F8;
}
.topbar {
  height: 56px;
  flex-shrink: 0;
  background: #fff;
  border-bottom: 1px solid #E6E8EF;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 clamp(12px, 2vw, 24px);
  z-index: 10;
  gap: 12px;
  flex-wrap: nowrap;
}
.brand { display: flex; gap: 12px; align-items: center; flex-shrink: 0; }
.logo { width: 32px; height: 32px; border-radius: 8px; background: #2B5CFF; color: #fff; display: grid; place-items: center; font-weight: 700; flex-shrink:0; }
.title { font-weight: 700; font-size: 15px; line-height: 1; white-space: nowrap; }
.subtitle { font-size: 11px; color: #8A8FA3; margin-top: 2px; white-space: nowrap; }
.nav { display: flex; gap: 6px; flex-shrink: 0; }
.nav a { padding: 6px 14px; border-radius: 8px; text-decoration: none; color: #5A6075; font-size: 13px; font-weight: 500; white-space: nowrap; transition: background .15s, color .15s; }
.nav a:hover { background: #F3F4F8; color: #2B5CFF; }
.nav a.active { background: #EEF0FF; color: #2B5CFF; }

/* ---- 主内容区：唯一滚动容器，像原生软件一样 ---- */
.main {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  overflow-x: hidden;
  width: 100%;
  margin: 0;
  padding: clamp(12px,2vw,24px) clamp(12px,2vw,24px) 24px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  /* 让路由页面填满剩余高度，便于内部再分栏滚动 */
  scrollbar-width: thin;
  scrollbar-color: #D5D8E2 transparent;
  scrollbar-gutter: stable;
}
.main > * {
  flex: 0 0 auto;
  width: 100%;
  min-height: 0;
  display: flex;
  flex-direction: column;
}
/* 当页面需要占满视口（如流水页），由页面自身 flex:1 撑开 */
.main > .tx-page,
.main > .stats-page,
.main > .settings-page {
  flex: 1 1 auto;
}

/* 优雅的桌面端滚动条 - 只在 .main 与内部滚动容器生效 */
.main::-webkit-scrollbar,
.table-wrap::-webkit-scrollbar,
.n-data-table-wrapper::-webkit-scrollbar,
.n-drawer-body::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}
.main::-webkit-scrollbar-track,
.table-wrap::-webkit-scrollbar-track {
  background: transparent;
}
.main::-webkit-scrollbar-thumb,
.table-wrap::-webkit-scrollbar-thumb,
.n-data-table-wrapper::-webkit-scrollbar-thumb {
  background: #D5D8E2;
  border-radius: 8px;
  border: 2px solid transparent;
  background-clip: content-box;
}
.main::-webkit-scrollbar-thumb:hover,
.table-wrap::-webkit-scrollbar-thumb:hover {
  background: #B8BDCB;
  background-clip: content-box;
}
/* 防止出现全局 page 滚动条 */
html::-webkit-scrollbar, body::-webkit-scrollbar { width: 0; height: 0; }

@media (max-width: 900px) { .main { scrollbar-gutter: auto; } }
@media (max-width: 720px) {
  .topbar { height: 52px; padding: 0 12px; gap: 8px; }
  .logo { width: 28px; height: 28px; font-size: 12px; }
  .title { font-size: 13px; }
  .subtitle { display: none; }
  .nav a { padding: 6px 10px; font-size: 12px; }
  .main { padding: 12px 12px 20px; }
}
@media (max-width: 480px) {
  .topbar { height: auto; min-height: 52px; padding: 8px 12px; }
  .nav { gap: 4px; }
  .nav a { padding: 5px 9px; }
}
</style>
