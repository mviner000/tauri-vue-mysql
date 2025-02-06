<!-- src/App.vue -->

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Button } from '@/components/ui/button';
import { ReloadIcon } from '@radix-icons/vue';
import { listen } from '@tauri-apps/api/event';
import SudoPasswordModal from '@/components/SudoPasswordModal.vue';

const mysqlStatus = ref<'checking' | 'installed' | 'not-installed'>('checking');
const isLoading = ref(true);
const isInstalling = ref(false);
const installError = ref<string | null>(null);

async function checkMySQL() {
  try {
    const isInstalled = await invoke<boolean>('is_mysql_installed');
    mysqlStatus.value = isInstalled ? 'installed' : 'not-installed';
  } catch (error) {
    console.error('Error checking MySQL:', error);
    mysqlStatus.value = 'not-installed';
  } finally {
    isLoading.value = false;
  }
}

async function handleInstall() {
  isInstalling.value = true;
  installError.value = null;
  
  try {
    const result = await invoke<boolean>('install_mysql');
    if (result) {
      // Wait a moment for services to initialize
      await new Promise(resolve => setTimeout(resolve, 2000));
      await checkMySQL();
    }
  } catch (error) {
    console.error('Installation failed:', error);
    installError.value = typeof error === 'string' ? error : 'Installation failed';
  } finally {
    isInstalling.value = false;
  }
}

// Add log display state
const installLogs = ref<string[]>([]);

// Listen for installation logs
listen('mysql-install-log', (event) => {
    installLogs.value.push(event.payload as string);
});

listen('mysql-install-error', (event) => {
    installLogs.value.push(`ERROR: ${event.payload}`);
});

onMounted(() => {
  checkMySQL();
});
</script>

<template>
  <div class="p-4 flex flex-col items-center gap-4">

    <!-- Add to your template -->
    <SudoPasswordModal />
    
    <div class="border rounded-md p-4 max-h-48 overflow-y-auto bg-gray-100">
        <div v-for="(log, index) in installLogs" :key="index" class="text-sm font-mono">
            {{ log }}
        </div>
    </div>

    <!-- Existing status display -->
    <div class="flex items-center gap-2">
      <Button variant="outline" @click="checkMySQL" :disabled="isLoading || isInstalling">
        <template v-if="isLoading || isInstalling">
          <ReloadIcon class="mr-2 h-4 w-4 animate-spin" />
          {{ isLoading ? 'Checking...' : 'Installing...' }}
        </template>
        <span v-else>Refresh Status</span>
      </Button>
    </div>

    <div v-if="!isLoading" class="flex flex-col items-center gap-4">
      <div class="flex items-center gap-2">
        <template v-if="mysqlStatus === 'installed'">
          <div class="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
          <span class="text-green-500">MySQL is installed</span>
        </template>
        
        <template v-else>
          <div class="w-2 h-2 rounded-full bg-red-500" />
          <span class="text-red-500">MySQL not found</span>
        </template>
      </div>

      <!-- Installation controls -->
      <div v-if="mysqlStatus === 'not-installed'" class="flex flex-col items-center gap-2">
        <Button 
          variant="default" 
          @click="handleInstall"
          :disabled="isInstalling"
        >
          <template v-if="isInstalling">
            <ReloadIcon class="mr-2 h-4 w-4 animate-spin" />
            Installing...
          </template>
          <span v-else>Install MySQL Automatically</span>
        </Button>

        <p v-if="installError" class="text-sm text-red-500 text-center">
          {{ installError }}<br>
          <span class="text-muted-foreground">Please check your permissions and internet connection</span>
        </p>
        
        <p class="text-sm text-muted-foreground text-center max-w-[400px]">
          This will install MySQL Server 8.0. Requires administrator privileges.
        </p>
      </div>
    </div>
  </div>
</template>