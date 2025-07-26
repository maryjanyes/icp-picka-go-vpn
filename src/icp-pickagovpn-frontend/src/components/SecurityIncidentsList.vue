<script>
    export default {
        methods: {
            prev() {
                //   
            },
            next() {
                //
            },
            applySearchFilter() {
                //
            }
        },
    }
</script>
<script setup>
    import { ref } from 'vue';
    
    const columnNames = {
        context: 'Context',
        timestamp: 'Timestamp',
        attackType: 'Attack type',
        threatLevel: 'Threat level',
        status: 'Attack status',
    };
    const incidentColumns = ['context', 'timestamp', 'attackType', 'threatLevel', 'status'];
    const incidents = ref([{
        context: 'Man in a middle attack',
        timestamp: Date.now(),
        attackType: 'ddos',
        threatLevel: 'high',
        status: 'blocked',
    }, {
        context: 'Tried deep script injection',
        timestamp: Date.now(),
        attackType: 'ddos',
        threatLevel: 'medium',
        status: 'blocked',
    }]);
</script>

<template>
  <div class="security-incidents-list-container flex flex-col gap-10 p-5 bg-cyan-900 rounded-lg">
    <div class="flex flex-row justify-between">
        <h2 class="text-white text-2sm font-semibold">Security Incidents Log</h2>
        <div class="flex flex-row gap-2">
            <input
                placeholder="Search by IP address"
                class="primary-input"
            />
            <button class="primary-button">Last 7 days</button>
        </div>
    </div>
    <div class="security-incidents-list p-6">
        <ul class="flex flex-col">
            <li>
                <ul class="flex flex-row gap-4">
                    <li v-for="c in incidentColumns" class="w-50">
                        <span class="text-white text-sm font-semibold">{{ columnNames[c] }}</span>
                    </li>
                </ul>
            </li>
            <li v-for="i in incidents">
                <ul class="flex flex-row gap-4">
                    <li v-for="c in incidentColumns" class="w-50">
                        <span class="text-white text-sm">{{ i[c] }}</span>
                    </li>
                </ul>
            </li>
        </ul>
    </div>
    <div>
        <p class="text-white text-sm mb-5">Showing 3 from 3 incidents</p>
        <div class="flex flex-row gap-2">
            <button class="primary-button" @click="prev">Previous</button>
            <button class="secondary-button" @click="next">Next</button>
        </div>
    </div>
  </div>
</template>

<style scoped>
    .connect-button {
        background-color: #7C32F3;
        padding: 4px 28px;
        max-height: 42px;
        border-radius: 12px;
        border: none;
        color: #fff;
        box-shadow: 0.5px solid #969696;
    }
</style>