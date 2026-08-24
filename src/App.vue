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
html, body { margin: 0; font-family: ui-sans-serif, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial; background: #F5F6F8; color: #1A1E2B; }
.app-layout { min-height: 100vh; display:flex; flex-direction:column; }
.topbar { height: 56px; background: #fff; border-bottom: 1px solid #E6E8EF; display: flex; align-items: center; justify-content: space-between; padding: 0 clamp(12px, 2vw, 24px); position: sticky; top: 0; z-index: 10; gap: 12px; flex-wrap: nowrap; }
.brand { display: flex; gap: 12px; align-items: center; flex-shrink: 0; }
.logo { width: 32px; height: 32px; border-radius: 8px; background: #2B5CFF; color: #fff; display: grid; place-items: center; font-weight: 700; flex-shrink:0; }
.title { font-weight: 700; font-size: 15px; line-height: 1; white-space: nowrap; }
.subtitle { font-size: 11px; color: #8A8FA3; margin-top: 2px; white-space: nowrap; }
.nav { display: flex; gap: 6px; flex-shrink: 0; }
.nav a { padding: 6px 14px; border-radius: 8px; text-decoration: none; color: #5A6075; font-size: 13px; font-weight: 500; white-space: nowrap; }
.nav a.active { background: #EEF0FF; color: #2B5CFF; }
.main { max-width: none; width: 100%; margin: 0; padding: clamp(12px,2vw,24px) clamp(12px,2vw,24px) 40px; flex:1; box-sizing: border-box; }
@media (max-width: 900px) { .main { max-width: none; } }
@media (max-width: 720px) {
  .topbar { height: 52px; padding: 0 12px; gap: 8px; }
  .logo { width: 28px; height: 28px; font-size: 12px; }
  .title { font-size: 13px; }
  .subtitle { display: none; }
  .nav a { padding: 6px 10px; font-size: 12px; }
  .main { padding: 12px 12px 32px; }
}
@media (max-width: 480px) {
  .topbar { height: auto; min-height: 52px; padding: 8px 12px; }
  .nav { gap: 4px; }
  .nav a { padding: 5px 9px; }
}
</style>