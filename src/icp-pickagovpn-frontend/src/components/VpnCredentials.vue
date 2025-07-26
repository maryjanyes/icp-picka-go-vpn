<script>
    import { icp_pickagovpn_backend } from '../../../declarations/icp-pickagovpn-backend/index';

    export default {
        methods: {
            joinVpn() {
                if (connectedIDVPNRole === 'CUSTOMER') {
                    icp_pickagovpn_backend.create_vault('CUSTOMER');
                } else {
                    icp_pickagovpn_backend.create_vault('RENTER');
                }
            },
            download() {
                if (connectedIDVPNRole === 'CUSTOMER') {
                    // todo: download client VPN
                } else {
                    // todo: download VPN command line tool
                }
            },
        },
    }
</script>
<script setup>
    import { ref } from 'vue';

    const props = defineProps({
        connectedIcpId: String,
    });
    const userVpnConnectedProperties = ref({
        role: null,
        dedicatedVpnIps: [],
        rentedVpnIps: [],
        credits: null,
    });
    const selectedVpnRole = ref('CUSTOMER');
</script>

<template>
  <div class="vpn-credentials-container w-full p-6 flex flex-row gap-10">
    <div class="p-8">
        <span class="text-white text-md font-semibold">
            Connected ICP Account: 
        </span>
        <span class="text-white text-sm">
            {{ props.connectedIcpId }}
        </span>
    </div>
    <div v-if="!userVpnConnectedProperties.role" class="flex flex-col gap-5">
        <p class="text-white text-sm font-semibold">It looks like you still not have active VPN Profile</p>
        <p class="text-white text-2sm font-bold">
            Join Picka VPN on ICP now and try 10 FREE credits on Testnet!
        </p>
        <div class="flex flex-row gap-5">
            <select v-model="selectedVpnRole" class="field-select">
                <option value="">Select Role</option>
                <option value="CUSTOMER">CUSTOMER</option>
                <option value="RENTER">RENTER</option>
            </select>
            <button class="primary-button" @click="joinVpn">Join</button>
        </div>
    </div>
    <div v-if="!!userVpnConnectedProperties.role">
        <div v-if="userVpnConnectedProperties.role == 'CUSTOMER'">
            <div>
                <p class="text-white text-md font-bold">Your rented VPN Client IP's</p>
                <p>{{ userVpnConnectedProperties.rentedVpnIps.join(',') }}</p>
                <i class="copy-icon"></i>
            </div>
            <div>
                <p class="text-white text-md font-bold">Download VPN Client</p>
                <button class="primary-button" @click="download">Download</button>
            </div>
        </div>
        <div v-if="userVpnConnectedProperties.role == 'RENTER'">
            <div>
                <p class="text-white text-md font-bold">Download VPN Server Command Line Tool</p>
                <button class="primary-button" @click="download">Download</button>
            </div>
            <div>
                <p class="text-white text-md font-bold">Your dedicated VPN Server IP's</p>
                <p>{{ userVpnConnectedProperties.dedicatedVpnIps.join(',') }}</p>
                <i class="copy-icon"></i>
            </div>
        </div>
    </div>
  </div>
</template>

<style scoped>
    .vpn-credentials-container {
        background-color: #000;
    }
</style>
