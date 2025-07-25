<script>
    export default {
        methods: {
            joinVpn() {
                console.log('role...', selectedVpnRole)
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

    const userAuthProperties = defineProps({
        connectedIcpId: String,
    });
    const userVpnConnectedProperties = ref({
        role: null,
        dedicatedVpnIps: [],
        rentedVpnIps: [],
        credits: null,
    });
    const selectedVpnRole = ref();
</script>

<template>
  <div class="vpn-credentials-container w-full p-6 flex flex-row gap-10">
    <div class="p-8">
        <span class="text-white text-md font-semibold">
            Connected ICP Account: 
        </span>
        <span class="text-white text-sm">
            {{ userAuthProperties.connectedIcpId }}
        </span>
    </div>
    <div v-if="!userVpnConnectedProperties.role" class="flex flex-col gap-5">
        <label class="text-white text-2sm font-bold">
            Join Picka VPN on ICP now and try 10 FREE credits on Testnet!
        </label>
        <div class="flex flex-row gap-5">
            <select v-model="selectedVpnRole" class="field-select">
                <option value="" selected>Select Role</option>
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
