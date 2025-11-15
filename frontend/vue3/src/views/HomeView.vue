<script setup>
import { ref, reactive, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import LanguageSwitcher from './LanguageSwitcher.vue'
import { httpReq } from '@/assets/tools'

const { t, tm, rt } = useI18n()
const router = useRouter()
const setFormVisible = ref(false)
const workflow = reactive({ id: '', name: '', canvas: '' })

onMounted(() => {
  httpReq('GET', 'workflow', null, null, null).then((res) => {
    console.log(res)
  })
})

const newWorkflow = () => {
  workflow.id = ''
  workflow.name = ''
  workflow.canvas = ''
  setFormVisible.value = true
}

const createWorkflow = async () => {
  const r = await httpReq('POST', 'workflow', null, workflow, null)
  if (r && r.status === 200) {
    router.push({ name: 'workflow', params: { workflowId: r.data.id } })
  }
}
</script>

<template>
  <LanguageSwitcher />
  <h1>Workflows</h1>
  <router-link to="/workflow/editor">Workflow</router-link>
  <el-button type="primary" class="ml-2" @click="newWorkflow()">{{ $t('mainflow.add') }}</el-button>
  <el-table :data="tableData" stripe style="width: 100%">
    <el-table-column prop="id" label="Id" width="240" />
    <!-- <el-table-column prop="name" :label="tm('mainflow.table')[0]" width="500" /> -->
    <el-table-column :label="tm('mainflow.table')[0]" width="500">
      <template #default="scope">
        <el-text
          style="cursor: pointer"
          type="primary"
          size="large"
          @click="toSubflow(scope.$index, scope.row)"
        >
          {{ scope.row.name }}
        </el-text>
      </template>
    </el-table-column>

    <!-- <el-table-column prop="enabled" :label="tm('mainflow.table')[1]" width="80" /> -->
    <el-table-column fixed="right" :label="tm('mainflow.table')[2]" min-width="40">
      <template #default="scope">
        <el-button link type="primary" @click="toSubflow(scope.$index, scope.row)">
          {{ $t('common.edit') }}
        </el-button>
        |
        <el-button link type="primary" @click="editMainFlow(scope.$index, scope.row)">
          {{ $t('common.changeName') }}
        </el-button>
        |
        <el-button link type="danger" @click="deleteMainFlow(scope.$index, scope.row)">
          {{ $t('common.del') }}
        </el-button>
      </template>
    </el-table-column>
  </el-table>
  <el-dialog v-model="setFormVisible" :title="$t('workflow.form.title')" width="60%">
    <el-form :model="nodeData">
      <el-form-item :label="$t('mainflow.form.name')" :label-width="formLabelWidth">
        <el-input v-model="mainFlowData.name" autocomplete="off" />
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button type="primary" :loading="loading" @click="createWorkflow()">{{
        $t('common.save')
      }}</el-button>
      <el-button @click="hideForm()">{{ $t('common.cancel') }}</el-button>
    </template>
  </el-dialog>
</template>
