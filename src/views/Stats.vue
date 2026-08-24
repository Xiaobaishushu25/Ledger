<template>
  <div>
    <n-card size="small">
      <div style="font-weight:700; font-size:14px;">收支总览</div>
      <div class="stat-grid">
        <n-statistic label="累计收入" :value="formatMoney(stats.total_income)" />
        <n-statistic label="累计支出" :value="formatMoney(stats.total_expense)" />
        <n-statistic label="当前余额" :value="formatMoney(stats.balance)" />
      </div>
    </n-card>

    <n-card size="small" style="margin-top:12px">
      <div style="font-weight:600; font-size:13px;">按类别汇总</div>
      <n-empty v-if="!stats.by_category.length" description="暂无数据" style="margin-top:12px" />
      <div v-else style="margin-top:10px; display:flex; flex-direction:column; gap:8px">
        <div v-for="c in stats.by_category" :key="c.name" class="row">
          <span>{{ c.name }} <n-tag size="small" :type="c.type==='income'?'error':'success'" style="margin-left:6px">{{ c.type==="income"?"收入":"支出" }}</n-tag></span>
          <span :style="{color: c.type==='income'?'#D03050':'#18A058', fontWeight:700}">{{ formatMoney(c.total) }}</span>
        </div>
      </div>
    </n-card>

    <n-card size="small" style="margin-top:12px">
      <div style="font-weight:600; font-size:13px;">按月汇总</div>
      <n-empty v-if="!stats.by_month.length" description="暂无数据" style="margin-top:12px" />
      <div v-else style="margin-top:10px; display:flex; flex-direction:column; gap:8px">
        <div v-for="m in stats.by_month" :key="m.month" class="row">
          <span style="font-weight:600">{{ m.month }}</span>
          <span><span style="color:#D03050">+{{ formatMoney(m.income) }}</span> <span style="color:#18A058; margin-left:8px">-{{ formatMoney(m.expense) }}</span></span>
        </div>
      </div>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { NCard, NStatistic, NTag, NEmpty } from "naive-ui";
import { api, type Stats } from "../api/tauri";
import { formatMoney } from "../utils/format";

const stats = ref<Stats>({ total_income:0, total_expense:0, balance:0, by_category:[], by_month:[] });
onMounted(async ()=> { stats.value = await api.getStats(); });
</script>

<style scoped>
.stat-grid { display:grid; grid-template-columns: repeat(3, 1fr); gap:12px; margin-top:12px; }
.row { display:flex; justify-content:space-between; align-items:center; padding:10px 12px; background:#F9FAFB; border-radius:8px; border:1px solid #EEF0F3; gap:8px; flex-wrap: wrap; }
@media (max-width: 900px) { .stat-grid { grid-template-columns: repeat(2, 1fr); } }
@media (max-width: 560px) { .stat-grid { grid-template-columns: 1fr; } .row { padding: 9px 10px; } }
</style>