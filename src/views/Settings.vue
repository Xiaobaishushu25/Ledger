<template>
  <div class="settings-page">
    <n-card size="small">
      <div style="display:flex; justify-content:space-between; align-items:center">
        <div style="font-weight:700">类别管理</div>
        <n-button size="small" type="primary" @click="showAdd=true">新增类别</n-button>
      </div>
      <div style="margin-top:12px; display:flex; flex-wrap:wrap; gap:8px">
        <n-tag v-for="c in categories" :key="c.id" :type="c.type==='income'?'success':'error'" closable @close="handleDelete(c)" @click="handleEdit(c)" style="cursor:pointer">
          {{ c.name }}
        </n-tag>
      </div>
      <div style="margin-top:8px; font-size:11px; color:#8A8FA3">提示：点击可重命名，× 删除（被使用中的类别不可删）</div>
    </n-card>

    <n-card size="small" style="margin-top:12px">
      <div style="font-weight:600">数据管理</div>
      <div style="margin-top:10px; display:flex; gap:8px">
        <n-button @click="handleExportAll">导出全部 Excel</n-button>
      </div>
      <div style="margin-top:8px; font-size:11px; color:#888">数据库文件位于 软件同级 data 目录（exe所在目录/data/lab-ledger.db），图片存于 data/images/。导出文件将保存到系统下载目录。绿色便携，拷贝整个文件夹即可迁移。</div>
    </n-card>

    <n-card size="small" style="margin-top:12px">
      <div style="font-weight:600">关于</div>
      <div style="margin-top:6px; font-size:13px; color:#555; line-height:1.6">
        实验室极简记账 · Tauri + Vue3 + SQLite<br/>
        单机单人 · 零起累计余额 · 本地存储多图附件<br/>
        v0.1.0
      </div>
    </n-card>

    <n-modal v-model:show="showAdd" preset="dialog" title="新增类别" :show-icon="false">
      <n-input v-model:value="newName" placeholder="类别名称，如：材料费" />
      <n-radio-group v-model:value="newType" style="margin-top:10px">
        <n-space>
          <n-radio value="expense">支出</n-radio>
          <n-radio value="income">收入</n-radio>
        </n-space>
      </n-radio-group>
      <template #action>
        <n-button @click="showAdd=false">取消</n-button>
        <n-button type="primary" @click="doCreate">创建</n-button>
      </template>
    </n-modal>

    <n-modal v-model:show="showEdit" preset="dialog" title="重命名类别" :show-icon="false">
      <n-input v-model:value="editName" placeholder="新名称" />
      <template #action>
        <n-button @click="showEdit=false">取消</n-button>
        <n-button type="primary" @click="doRename">保存</n-button>
      </template>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { NCard, NTag, NButton, NModal, NInput, NRadioGroup, NRadio, NSpace, useMessage, useDialog } from "naive-ui";
import { api, type Category } from "../api/tauri";

const message = useMessage();
const dialog = useDialog();
const categories = ref<Category[]>([]);
const showAdd = ref(false);
const newName = ref("");
const newType = ref<"income"|"expense">("expense");
const showEdit = ref(false);
const editName = ref("");
let editTarget: Category | null = null;

async function load(){ categories.value = await api.listCategories(); }
async function doCreate(){
  if(!newName.value.trim()) return message.warning("请输入名称");
  try{ await api.createCategory(newName.value.trim(), newType.value); message.success("已创建"); showAdd.value=false; newName.value=""; load(); } catch(e:any){ message.error(String(e)); }
}
function handleEdit(c: Category){ editTarget=c; editName.value=c.name; showEdit.value=true; }
async function doRename(){
  if(!editTarget) return;
  if(!editName.value.trim()) return message.warning("请输入名称");
  try{ await api.updateCategory(editTarget.id, editName.value.trim()); message.success("已更新"); showEdit.value=false; load(); }catch(e:any){ message.error(String(e)); }
}
function handleDelete(c: Category){
  dialog.warning({ title:"确认删除", content:`删除类别「${c.name}」？若已有流水使用则会失败。`, positiveText:"删除", negativeText:"取消",
    onPositiveClick: async()=>{ try{ await api.deleteCategory(c.id); message.success("已删除"); load(); }catch(e:any){ message.error(String(e)); } } });
}
async function handleExportAll(){
  try{ const p = await api.exportExcel({}); message.success("已导出: " + p); }catch(e:any){ message.error(String(e)); }
}
onMounted(load);
</script>

