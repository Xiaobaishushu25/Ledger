<template>
  <div>
    <!-- Summary Cards -->
    <div class="cards">
      <n-card size="small" class="card">
        <div class="card-label">当前余额</div>
        <div class="card-value">{{ formatMoney(summary.balance) }}</div>
        <div class="card-hint">零起累计 · 收入-支出</div>
      </n-card>
      <n-card size="small" class="card income">
        <div class="card-label">本月收入</div>
        <div class="card-value" style="color:#D03050">{{ formatMoney(summary.month_income) }}</div>
        <div class="card-hint">{{ monthLabel }} 收入</div>
      </n-card>
      <n-card size="small" class="card expense">
        <div class="card-label">本月支出</div>
        <div class="card-value" style="color:#18A058">{{ formatMoney(summary.month_expense) }}</div>
        <div class="card-hint">{{ monthLabel }} 支出</div>
      </n-card>
    </div>

    <!-- Filter Bar -->
    <n-card size="small" style="margin-top:14px">
      <div class="filter-bar">
        <n-input v-model:value="filter.keyword" placeholder="搜索说明 / 备注" clearable class="f-keyword" @update:value="load" />
        <n-select v-model:value="filter.type_" placeholder="收支" clearable :options="typeOpts" class="f-type" @update:value="load" />
        <n-select v-model:value="filter.category_id" placeholder="类别" clearable :options="catOpts" class="f-cat" @update:value="load" />
        <n-date-picker v-model:value="filter.range" type="daterange" clearable class="f-range" @update:value="load" />
        <div class="f-spacer"></div>
        <n-button type="primary" @click="showForm = true; editing = null">记一笔</n-button>
        <n-button @click="handleExport">导出 Excel</n-button>
      </div>
    </n-card>

    <!-- Table -->
    <n-card size="small" style="margin-top:12px">
      <div class="table-wrap">
        <n-data-table :columns="columns" :data="items" :pagination="pagination" :loading="loading" :row-key="(r:any)=>r.id" size="small" :scroll-x="900" />
      </div>
    </n-card>

    <!-- Form Drawer -->
    <TransactionForm v-model:show="showForm" :editing="editing" :categories="categories" @saved="onSaved" />

    <!-- Detail Drawer for images/note -->
    <n-drawer v-model:show="showDetail" :width="isNarrow ? 340 : 420">
      <n-drawer-content :title="detail?.note || `明细 #${detail?.id}`" closable>
        <n-descriptions label-placement="left" bordered :column="1" size="small">
          <n-descriptions-item label="日期">{{ detail?.date }}</n-descriptions-item>
          <n-descriptions-item label="收支">{{ detail?.type === 'income' ? '收入' : '支出' }}</n-descriptions-item>
          <n-descriptions-item label="类别">{{ detail?.category_name || '—' }}</n-descriptions-item>
          <n-descriptions-item label="金额">{{ detail ? formatMoney(detail.amount) : '' }}</n-descriptions-item>
          <n-descriptions-item label="说明">{{ detail?.note || '—' }}</n-descriptions-item>
        </n-descriptions>
        <div style="margin-top:16px; font-weight:600; font-size:13px;">图片附件</div>
        <n-empty v-if="!detailImages.length" description="暂无图片" style="margin-top:12px" />
        <div v-else class="img-grid">
          <div v-for="img in detailImages" :key="img.id" class="img-item">
            <img :src="imgUrl(img.file_path)" @dblclick="openPreview(imgUrl(img.file_path))" title="双击放大" style="width:100%; height:120px; object-fit:cover; border-radius:8px; border:1px solid #eee; cursor: zoom-in" />
            <div style="font-size:11px; color:#888; margin-top:4px; word-break:break-all;">{{ img.file_name }}</div>
          </div>
        </div>
      </n-drawer-content>
    </n-drawer>
    <n-modal v-model:show="showPreview" preset="card" title="图片预览" style="width:90%; max-width:860px">
      <img :src="previewSrc" style="width:100%; max-height:75vh; object-fit:contain; border-radius:8px" />
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, h, onMounted, onUnmounted } from "vue";
import { NCard, NButton, NInput, NSelect, NDatePicker, NDataTable, NTag, NDrawer, NDrawerContent, NDescriptions, NDescriptionsItem, NEmpty, NModal, useMessage, useDialog } from "naive-ui";
import { api, type Category } from "../api/tauri";
import { formatMoney } from "../utils/format";
import TransactionForm from "../components/TransactionForm.vue";
import { convertFileSrc } from "@tauri-apps/api/core";

const message = useMessage();
const dialog = useDialog();

const isNarrow = ref(typeof window !== "undefined" ? window.innerWidth < 640 : false);
const onResize = () => { isNarrow.value = window.innerWidth < 640; };
onMounted(() => window.addEventListener("resize", onResize));
onUnmounted(() => window.removeEventListener("resize", onResize));

const summary = ref({ balance: 0, month_income: 0, month_expense: 0 });
const categories = ref<Category[]>([]);
const items = ref<any[]>([]);
const loading = ref(false);
const showForm = ref(false);
const editing = ref<any>(null);
const showDetail = ref(false);
const detail = ref<any>(null);
const detailImages = ref<any[]>([]);
const previewSrc = ref("");
const showPreview = ref(false);
function openPreview(src:string){ previewSrc.value = src; showPreview.value = true; }
const total = ref(0);
const page = ref(1);
const pageSize = ref(15);

const filter = ref({ keyword: "", type_: null as any, category_id: null as any, range: null as any });

const monthLabel = computed(()=> new Date().toISOString().slice(0,7));

const typeOpts = [{ label: "收入", value: "income" }, { label: "支出", value: "expense" }];
const catOpts = computed(()=> categories.value.map(c=> ({ label: c.name, value: c.id })));

const pagination = computed(()=> ({
  page: page.value,
  pageSize: pageSize.value,
  itemCount: total.value,
  showSizePicker: true,
  pageSizes: [10,15,30],
  onChange: (p:number)=> { page.value=p; load(); },
  onUpdatePageSize: (ps:number)=> { pageSize.value=ps; page.value=1; load(); },
  prefix: (info:any)=> `共 ${info.itemCount||0} 笔`
}));

function imgUrl(p: string){
  try { return convertFileSrc(p); } catch { return p; }
}

const columns:any = [
  { title: "日期", key: "date", width: 110, render: (r:any)=> r.date },
  { title: "收支", key: "type", width: 70, render: (r:any)=> h(NTag, { type: r.type==="income"?"error":"success", size:"small", bordered:false }, { default: ()=> r.type==="income"?"收入":"支出" }) },
  { title: "类别", key: "category_name", width: 110, render: (r:any)=> r.category_name || "—" },
  { title: "金额", key: "amount", width: 120, render: (r:any)=> h("span", { style: { color: r.type==="income"?"#D03050":"#18A058", fontWeight: 700 } }, formatMoney(r.amount)) },
  { title: "余额", key: "balance", width: 120, render: (r:any)=> h("span", { style:{ color:"#333", fontWeight:600 } }, r.balance!=null?formatMoney(r.balance):"—") },
  { title: "说明", key: "note", ellipsis: { tooltip: true }, render: (r:any)=> r.note || "—" },
  { title: "图片", key: "image_count", width: 70, render: (r:any)=> r.image_count ? h(NTag,{size:"small"}, {default:()=> `×${r.image_count}`}) : "—" },
  { title: "操作", key: "actions", width: 140, fixed: "right" as const, render: (r:any)=> h("div", { style:"display:flex; gap:6px" }, [
      h(NButton,{ size:"tiny", onClick: ()=> openDetail(r) }, { default: ()=> "查看" }),
      h(NButton,{ size:"tiny", onClick: ()=> { editing.value=r; showForm.value=true; } }, { default: ()=> "编辑" }),
      h(NButton,{ size:"tiny", type:"error", secondary:true, onClick: ()=> handleDelete(r) }, { default: ()=> "删除" }),
    ]) }
];

async function load(){
  loading.value=true;
  try {
    const range = filter.value.range;
    const params:any = {
      keyword: filter.value.keyword || null,
      type_: filter.value.type_ || null,
      category_id: filter.value.category_id || null,
      date_from: range?.[0] ? new Date(range[0]).toISOString().slice(0,10) : null,
      date_to: range?.[1] ? new Date(range[1]).toISOString().slice(0,10) : null,
      page: page.value, page_size: pageSize.value
    };
    const res = await api.listTransactions(params);
    items.value = res.items;
    total.value = res.total;
    summary.value.balance = res.balance;
    const s = await api.getDashboardSummary();
    summary.value.month_income = s.month_income;
    summary.value.month_expense = s.month_expense;
  } finally { loading.value=false; }
}

async function openDetail(row:any){
  detail.value=row;
  try{
    const d = await api.getTransaction(row.id);
    detailImages.value = (d as any).images || [];
  }catch{ detailImages.value=[]; }
  showDetail.value=true;
}
function handleDelete(row:any){
  dialog.warning({ title:"确认删除", content:`确定删除「${row.note||row.date} ${formatMoney(row.amount)}」？`, positiveText:"删除", negativeText:"取消",
    onPositiveClick: async()=>{ await api.deleteTransaction(row.id); message.success("已删除"); load(); } });
}
async function handleExport(){
  const range = filter.value.range;
  const f:any = {
    keyword: filter.value.keyword || null,
    type_: filter.value.type_ || null,
    category_id: filter.value.category_id || null,
    date_from: range?.[0] ? new Date(range[0]).toISOString().slice(0,10) : null,
    date_to: range?.[1] ? new Date(range[1]).toISOString().slice(0,10) : null,
  };
  try{ const p = await api.exportExcel(f); message.success("已导出: " + p); } catch(e:any){ message.error(String(e)); }
}
function onSaved(){ showForm.value=false; load(); }

onMounted(async ()=>{
  categories.value = await api.listCategories();
  load();
});
</script>

<style scoped>
.cards { display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px; }
.card-label { font-size: 11px; color:#8A8FA3; letter-spacing: .04em; }
.card-value { font-size: clamp(16px, 2.2vw, 22px); font-weight: 800; margin-top: 6px; word-break: break-all; }
.card-hint { font-size: 11px; color:#A0A6B8; margin-top: 4px; }
.filter-bar { display: flex; gap: 8px; align-items: center; flex-wrap: wrap; }
.f-keyword { flex: 1 1 180px; min-width: 160px; max-width: 260px; }
.f-type { flex: 0 1 120px; min-width: 110px; }
.f-cat { flex: 0 1 140px; min-width: 120px; }
.f-range { flex: 1 1 240px; min-width: 200px; max-width: 300px; }
.f-spacer { flex: 1 0 8px; min-width: 0; }
.table-wrap { width: 100%; overflow-x: auto; }
.img-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 10px; margin-top: 10px; }
@media (max-width: 900px) {
  .cards { grid-template-columns: repeat(2, 1fr); }
  .f-spacer { display: none; }
  .filter-bar { row-gap: 10px; }
}
@media (max-width: 560px) {
  .cards { grid-template-columns: 1fr; }
  .f-keyword, .f-type, .f-cat, .f-range { flex: 1 1 100%; max-width: 100%; min-width: 0; }
  .filter-bar .n-button { flex: 1 1 48%; }
  .img-grid { grid-template-columns: 1fr; }
}
</style>