<script setup lang="ts">
import {ref, onMounted} from 'vue';
import {FilterMatchMode} from '@primevue/core/api';
import type {User} from "@/scripts/definitions";
import {createUser, deleteUser, fetchUsers, patchUser} from "@/scripts/data.js";
import {useUserStore} from "@/stores/user.js";
import ProfilePicture from "@/views/components/ProfilePicture.vue";
import {useToast} from "primevue/usetoast";

const userStore = useUserStore();
const toast = useToast();

const users = ref<User[]>();
const filters = ref({
  global: {value: null, matchMode: FilterMatchMode.CONTAINS},
  name: {value: null, matchMode: FilterMatchMode.CONTAINS},
  firstname: {value: null, matchMode: FilterMatchMode.CONTAINS},
  lastname: {value: null, matchMode: FilterMatchMode.CONTAINS},
  email: {value: null, matchMode: FilterMatchMode.CONTAINS},
  admin: {value: null, matchMode: FilterMatchMode.EQUALS}
});

const loading = ref(true);
const isEdit = ref(true);
const submitted = ref(false);
const editDialog = ref(false);
const deleteDialog = ref(false);
const selectedUser = ref<User | null | {}>(null);

onMounted(async () => {
  users.value = await fetchUsers(userStore.token);
  loading.value = false;
});

const editUserDialog = (user: User) => {
  selectedUser.value = user;
  isEdit.value = true
  editDialog.value = true;
};

function createUserDialog() {
  isEdit.value = false;
  editDialog.value = true;
  selectedUser.value = {}
}

async function closeEditDialog() {
  editDialog.value = false;
  selectedUser.value = null;
}

async function closeDeleteDialog() {
  deleteDialog.value = false;
  selectedUser.value = null;
}

async function saveEdit() {
  submitted.value = true;
  if (!selectedUser.value.email?.trim() || !selectedUser.value.name?.trim()) {
    return;
  }

  if (isEdit.value) {
    loading.value = true;
    await patchUser(selectedUser.value, userStore.token)
    toast.add({severity: 'success', summary: 'Successful', detail: 'User Edited', life: 3000});
    users.value = await fetchUsers(userStore.token);
    loading.value = false;
  } else {
    loading.value = true;
    selectedUser.value.password = "Goon";
    await createUser(selectedUser.value, userStore.token)
    toast.add({severity: 'success', summary: 'Successful', detail: 'User Created', life: 3000});
    users.value = await fetchUsers(userStore.token);
    loading.value = false;
  }

  editDialog.value = false;
  selectedUser.value = null;
  submitted.value = false;
}

const confirmDeleteUser = (user: User) => {
  if (user.id == userStore.user?.id) {
    toast.add({severity: 'error', summary: 'Not allowed', detail: 'Can\'t delete yourself!', life: 3000});
    return;
  }
  selectedUser.value = user;
  deleteDialog.value = true;
};

async function removeUser() {
  loading.value = true;
  await deleteUser(selectedUser.value, userStore.token);
  users.value = await fetchUsers(userStore.token);
  loading.value = false;
  deleteDialog.value = false;
  selectedUser.value = null;
  toast.add({severity: 'success', summary: 'Successful', detail: 'User Deleted', life: 3000});
}

</script>


<template>

  <div class="bg-surface-900 p-4 rounded-md max-w-[calc(100vw-345px)]">
    <DataTable v-model:filters="filters" :value="users" paginator scrollable :rows="10" dataKey="id" filterDisplay="row"
               :loading="loading"
               :globalFilterFields="['name', 'firstname','lastname', 'email', 'admin']"
    >
      <template #header>
        <Button label="Create User" severity="success" icon="pi pi-plus" @click="createUserDialog"/>
        <div class="flex justify-end">
          <IconField>
            <InputIcon>
              <i class="pi pi-search"/>
            </InputIcon>
            <InputText v-model="filters['global'].value" placeholder="Keyword Search"/>
          </IconField>
        </div>
      </template>
      <template #empty> No Users found.</template>
      <template #loading> Loading Users. Please wait.</template>
      <Column field="name" sortable header="Username" style="min-width: 9rem">
        <template #body="{ data }" #editor="">
          <div class="flex items-center gap-2">
            <ProfilePicture :user="data.id" class="w-12 h-12 rounded-full"></ProfilePicture>
            <span>{{ data.name }}</span>
          </div>
        </template>
        <template #filter="{ filterModel, filterCallback }">
          <InputText v-model="filterModel.value" type="text" @input="filterCallback()"
                     placeholder="Search by username"/>
        </template>
      </Column>
      <Column field="firstname" sortable header="Firstname" style="min-width: 4rem">
        <template #body="{ data }">
          <div class="flex items-center gap-2">
            <span>{{ data.firstname }}</span>
          </div>
        </template>
        <template #filter="{ filterModel, filterCallback }">
          <InputText v-model="filterModel.value" type="text" @input="filterCallback()"
                     placeholder="Search by firstname"/>
        </template>
      </Column>
      <Column field="lastname" sortable header="Lastname" style="min-width: 4rem">
        <template #body="{ data }">
          <div class="flex items-center gap-2">
            <span>{{ data.lastname }}</span>
          </div>
        </template>
        <template #filter="{ filterModel, filterCallback }">
          <InputText v-model="filterModel.value" type="text" @input="filterCallback()"
                     placeholder="Search by lastname"/>
        </template>
      </Column>
      <Column field="email" sortable header="E-Mail" style="min-width: 24rem">
        <template #body="{ data }">
          <span>{{ data.email }}</span>
        </template>
        <template #filter="{ filterModel, filterCallback }">
          <InputText v-model="filterModel.value" type="text" @input="filterCallback()" placeholder="Search by country"/>
        </template>
      </Column>
      <Column field="admin" sortable header="Admin" dataType="boolean" style="min-width: 6rem">
        <template #body="{ data }">
          <i class="pi"
             :class="{ 'pi-check-circle text-green-500': data.admin, 'pi-times-circle text-red-400': !data.admin }"></i>
        </template>
        <template #filter="{ filterModel, filterCallback }">
          <Checkbox v-model="filterModel.value" :indeterminate="filterModel.value === null" binary
                    @change="filterCallback()"/>
        </template>
      </Column>
      <Column>
        <template #body="slotProps">
          <Button icon="pi pi-pencil" outlined rounded class="mr-2" @click="editUserDialog(slotProps.data)"/>
          <Button icon="pi pi-trash" outlined rounded severity="danger" @click="confirmDeleteUser(slotProps.data)"/>
        </template>
      </Column>
    </DataTable>
    <Dialog v-model:visible="deleteDialog" :style="{ width: '400px' }" header="Delete User" :modal="true">
      <div class="flex flex-col items-center justify-center">
        <div class="border border-gray-500 border-dashed bg-gray-800 rounded">
          <ProfilePicture class="h-44 w-44 rounded" :user="selectedUser?.id"></ProfilePicture>
          <p class="text-gray-100 flex justify-center text-xl font-semibold">{{ selectedUser?.name }}</p>
        </div>
        <p class="text-red-400 font-semibold my-2 text-xl">Do you really want to delete this user</p>
      </div>
      <template #footer>
        <Button label="Cancel" icon="pi pi-times" text @click="closeDeleteDialog"/>
        <Button label="Delete" icon="pi pi-check" severity="danger" @click="removeUser"/>
      </template>
    </Dialog>
    <Dialog v-model:visible="editDialog" :style="{ width: '450px' }" header="User Details" :modal="true">
      <template #header>
        <span v-if="isEdit" class="font-bold text-2xl">Edit User</span>
        <span v-else class="font-bold text-2xl">Create User</span>
      </template>
      <div class="flex flex-col gap-6">
        <div v-if="isEdit" class="p-4 border bg-gray-800 rounded-md border-dashed">
          <ProfilePicture :user="selectedUser?.id" class="block m-auto rounded-md "/>
        </div>
        <div class="grid grid-cols-4 gap-2">
          <div class="col-span-4">
            <label for="name" class="block font-bold mb-3">Username</label>
            <InputText id="name" v-model.trim="selectedUser.name" required="true" autofocus
                       :invalid="submitted && !selectedUser.name" fluid/>
            <small v-if="submitted && !selectedUser.name" class="text-red-500">Name is required.</small>
          </div>

          <div class="col-span-2">
            <label for="firstname" class="font-bold mb-3">Firstname</label>
            <InputText id="firstname" v-model.trim="selectedUser.firstname" fluid/>
          </div>

          <div class="col-span-2">
            <label for="lastname" class="font-bold mb-3">Lastname</label>
            <InputText id="lastname" v-model.trim="selectedUser.lastname" fluid/>
          </div>
          <div class="col-span-3">
            <label for="email" class="font-bold mb-3">Email</label>
            <InputText id="email" v-model.trim="selectedUser.email" required="true"
                       :invalid="submitted && !selectedUser.email" fluid/>
            <small v-if="submitted && !selectedUser.email" class="text-red-500">Email is required.</small>
          </div>
          <div class="col-span-1">
            <label for="isadmin" class="block font-bold mb-3">Is Admin</label>
            <div class="flex items-center">
              <Checkbox id="isadmin" v-model="selectedUser.admin" :binary="true"/>
            </div>
          </div>
        </div>
      </div>

      <template #footer>
        <Button label="Cancel" icon="pi pi-times" text @click="closeEditDialog"/>
        <Button label="Save" icon="pi pi-check" @click="saveEdit"/>
      </template>
    </Dialog>
  </div>
</template>
