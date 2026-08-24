<template>
  <n-drawer v-model:show="visible" :width="drawerWidth" :trap-focus="false">
    <n-drawer-content :title="editing ? `编辑 #${editing.id}` : `记一笔`" closable>
      <n-form :model="form" label-placement="left" :label-width="isNarrow ? 56 : 64" size="small">
        <n-form-item label="日期">
          <n-date-picker v-model:value="form.dateTs" type="date" style="width:100%" />
        </n-form-item>
        <n-form-item label="收支">
          <n-radio-group v-model:value="form.type_">
            <n-space>
              <n-radio value="expense">支出</n-radio>
              <n-radio value="income">收入</n-radio>
            </n-space>
          </n-radio-group>
        </n-form-item>
        <n-form-item label="金额">
          <n-input-number v-model:value="form.amount" :min="0.01" :precision="2" placeholder="0.00" style="width:100%">
            <template #prefix>¥</template>
          </n-input-number>
        </n-form-item>
        <n-form-item label="类别">
          <n-select v-model:value="form.category_id" :options="catOptions" placeholder="选择类别" clearable />
        </n-form-item>
        <n-form-item label="说明">
          <n-input v-model:value="form.note" type="textarea" :autosize="{minRows:2, maxRows:4}" placeholder="如：购买离心管一批" />
        </n-form-item>
        <n-form-item label="图片">
          <div style="width:100%">
            <div v-if="existingImages.length" style="margin-bottom:10px">
              <div style="font-size:12px; color:#666; margin-bottom:6px">已有附件（{{ existingImages.length }}）· 点击×删除 · 双击放大</div>
              <div style="display:grid; grid-template-columns: repeat(3, 1fr); gap:8px">
                <div v-for="img in existingImages" :key="img.id" style="position:relative; border:1px solid #eee; border-radius:8px; overflow:hidden; background:#fafafa">
                  <img :src="img.url" :alt="img.file_name" style="width:100%; height:90px; object-fit:cover; display:block; cursor:zoom-in" @dblclick="previewExisting(img.url)" />
                  <div style="padding:4px 6px; font-size:10px; color:#666; white-space:nowrap; overflow:hidden; text-overflow:ellipsis">{{ img.file_name }}</div>
                  <n-button size="tiny" type="error" secondary circle style="position:absolute; top:4px; right:4px" @click="handleRemoveExisting(img.id)">✕</n-button>
                </div>
              </div>
            </div>
            <n-upload :file-list="fileList" :max="20" list-type="image-card" :on-update:file-list="onFileList" :custom-request="()=>{}" accept="image/*" multiple>
              点击或拖拽上传
            </n-upload>
            <div style="font-size:11px; color:#888; margin-top:6px">支持多图，支持粘贴上传（截图后 Ctrl+V）· 图片将保存到软件同级 data/images/ 目录</div>
            <div v-if="pendingFiles.length" style="font-size:11px; color:#2B5CFF; margin-top:4px">待上传 {{ pendingFiles.length }} 张（保存时写入）</div>
          </div>
        </n-form-item>
      </n-form>
      <template #footer>
        <n-space justify="end">
          <n-button @click="visible=false">取消</n-button>
          <n-button type="primary" :loading="saving" @click="handleSave">保存</n-button>
        </n-space>
      </template>
    </n-drawer-content>
  </n-drawer>
  <n-modal v-model:show="showPreview" preset="card" title="图片预览" style="width:90%; max-width:860px">
    <img :src="previewSrc" style="width:100%; max-height:75vh; object-fit:contain; border-radius:8px" />
  </n-modal>
</template>

<script setup lang="ts">
import { ref, watch, computed, onMounted, onUnmounted } from "vue";
import { NDrawer, NDrawerContent, NForm, NFormItem, NInput, NInputNumber, NSelect, NDatePicker, NRadioGroup, NRadio, NSpace, NButton, NUpload, NModal, useMessage } from "naive-ui";
import { api, type Category } from "../api/tauri";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
const props = defineProps<{ show: boolean; editing: any; categories: Category[] }>();
const emit = defineEmits(["update:show","saved"]);
const message = useMessage();
const visible = computed({ get: ()=> props.show, set: v=> emit("update:show", v) });
const saving = ref(false);
const form = ref({ dateTs: Date.now(), type_: "expense" as "income"|"expense", amount: null as any, category_id: null as any, note: "" });
const fileList = ref<any[]>([]);
const pendingFiles = ref<File[]>([]);
const existingImages = ref<{id:number; file_path:string; file_name:string; url:string}[]>([]);
const showPreview = ref(false);
const previewSrc = ref("");
function previewExisting(url:string){ previewSrc.value=url; showPreview.value=true; }
const isNarrow = ref(typeof window !== "undefined" ? window.innerWidth < 480 : false);
const drawerWidth = computed(()=> isNarrow.value ? Math.min(360, Math.floor(window.innerWidth * 0.92)) : 440);
const onResize = ()=> { isNarrow.value = window.innerWidth < 480; };
function onFileList(list:any[]){ fileList.value = list; pendingFiles.value = list.filter(f=> f.file).map(f=> f.file as File); }
async function handleRemoveExisting(id:number){ try{ await invoke("delete_image", { image_id: id }); existingImages.value = existingImages.value.filter(x=> x.id!==id); message.success("已删除图片"); }catch(e:any){ message.error(String(e)); } }
function handlePaste(e: ClipboardEvent){
  if(!visible.value) return;
  const items = e.clipboardData?.items;
  if(!items) return;
  let added = 0;
  for(let i=0;i<items.length;i++){
    const it = items[i];
    if(it.kind === "file" && it.type.startsWith("image/")){
      const raw = it.getAsFile();
      if(!raw) continue;
      const ext = raw.type.split("/")[1] || "png";
      const name = raw.name && raw.name.trim() !== "" ? raw.name : `粘贴图片_${Date.now()}_${added}.${ext}`;
      const file = raw.name ? raw : new File([raw], name, { type: raw.type });
      const finalName = (file as File).name || name;
      const id = `${Date.now()}_${added}_${Math.random().toString(36).slice(2,6)}`;
      const url = URL.createObjectURL(file);
      const entry:any = { id, name: finalName, status: "finished", url, thumbnailUrl: url, file };
      fileList.value = [...fileList.value, entry];
      pendingFiles.value.push(file as File);
      added++;
    }
  }
  if(added>0){ e.preventDefault(); message.success(`已粘贴${added}张图片`); }
}
onMounted(()=>{ window.addEventListener("resize", onResize); window.addEventListener("paste", handlePaste); });
onUnmounted(()=>{ window.removeEventListener("resize", onResize); window.removeEventListener("paste", handlePaste); });
const catOptions = computed(()=> props.categories.filter(c=> c.type===form.value.type_).map(c=> ({ label: c.name, value: c.id })));
watch(()=> props.show, async (v)=>{
  if(v){
    if(props.editing){
      form.value = { dateTs: new Date(props.editing.date).getTime(), type_: props.editing.type, amount: props.editing.amount/100, category_id: props.editing.category_id, note: props.editing.note };
      fileList.value=[]; pendingFiles.value=[]; existingImages.value=[];
      try{ const d:any = await api.getTransaction(props.editing.id); const imgs = d.images || []; existingImages.value = imgs.map((it:any)=> ({ id: it.id, file_path: it.file_path, file_name: it.file_name, url: convertFileSrc(it.file_path) })); if(imgs.length) console.log("[loaded]", imgs); }catch(e:any){ console.error(e); message.error("加载图片失败: "+String(e)); }
    } else {
      form.value = { dateTs: Date.now(), type_: "expense", amount: null, category_id: null, note: "" };
      fileList.value=[]; pendingFiles.value=[]; existingImages.value=[];
    }
  }
});
async function fileToBytes(file: File): Promise<number[]>{ const buf = await file.arrayBuffer(); return Array.from(new Uint8Array(buf)); }
async function handleSave(){
  if(form.value.amount==null || form.value.amount<=0) return message.warning("请输入正确金额");
  const dateStr = new Date(form.value.dateTs).toISOString().slice(0,10);
  const payload:any = { date: dateStr, amount: form.value.amount, type_: form.value.type_, category_id: form.value.category_id, note: form.value.note };
  saving.value=true;
  try{ let id:number; if(props.editing){ await api.updateTransaction(props.editing.id, payload); id = props.editing.id; } else { id = await api.createTransaction(payload); } if(pendingFiles.value.length){ for(const f of pendingFiles.value){ const data = await fileToBytes(f); await invoke("save_image_bytes", { transaction_id: id, transactionId: id, file_name: f.name, fileName: f.name, data }); } } message.success("已保存"); emit("saved"); } catch(e:any){ message.error(String(e)); } finally{ saving.value=false; }
}
</script>
