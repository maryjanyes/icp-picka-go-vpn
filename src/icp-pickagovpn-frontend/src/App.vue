<script setup>
import SiteHeader from './components/Header.vue';
import SiteFooter from './components/Footer.vue';
import SecurityIncidentsList from './components/SecurityIncidentsList.vue';
import Stats from './components/Stats.vue';
import VpnCredentials from './components/VpnCredentials.vue';
// import { icp_picka_go_vpn_backend } from 'declarations/icp-picka-go-vpn-backend/index';

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  /* await icp_picka_go_vpn_backend.greet(name).then((response) => {
    greeting.value = response;
  }); */
}
</script>

<template>
  <main>
    <div class="flex flex-col gap-10 pl-15 pr-15">
      <SiteHeader />
      <VpnCredentials connected-icp-id="0x742d35Cc6634C0532925a3b844Bc454e4438f44e" />
      <Stats />
      <SecurityIncidentsList />
      <SiteFooter />
    </div>
  </main>
</template>
