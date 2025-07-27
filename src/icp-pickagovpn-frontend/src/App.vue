<script setup>
  import SiteHeader from './components/Header.vue';
  import SiteFooter from './components/Footer.vue';
  import SecurityIncidentsList from './components/SecurityIncidentsList.vue';
  import Stats from './components/Stats.vue';
  import VpnCredentials from './components/VpnCredentials.vue';
  import { ref } from 'vue';
  import { AuthClient } from '@dfinity/auth-client';
  import { createActor, canisterId } from '../declarations/icp-pickagovpn-backend';

  let authClient;
    let identity = ref(null);
    let actor;
    let isAuthenticated;

    const network = process.env.DFX_NETWORK;
    const identityProvider =
        network === 'ic'
            ? 'https://identity.ic0.app'
            : 'http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943';

  const updateClient = () => {
    AuthClient.create().then((client) => {
      authClient = client;
      identity = authClient.getIdentity();
      actor = createActor(canisterId, {
        agentOptions: {
          identity,
        },
      });
      isAuthenticated = authClient.isAuthenticated();
    });
  };

  updateClient();

  const connectIdentity = () => {
    authClient?.login({
      identityProvider,
      onSuccess: updateClient,
    }); 
  };
</script>

<template>
  <main>
    <div class="flex flex-col gap-10 pl-15 pr-15">
      <SiteHeader :connect="connectIdentity" />
      <VpnCredentials connected-icp-id="identity" />
      <Stats />
      <SecurityIncidentsList />
      <SiteFooter />
    </div>
  </main>
</template>
