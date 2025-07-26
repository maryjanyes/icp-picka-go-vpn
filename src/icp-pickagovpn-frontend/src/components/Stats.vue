<script setup>
    import { ref } from 'vue';
    
    const stats = ref({
        totalBlockedAttacks: {
            count: 120,
            percentage: 20,
        },
        activeThreats: {
            count: 80,
            percentage: 10,
        },
        effectivenessScore: {
            count: 8,
            percentage: 40,
        },
    });
</script>

<template>
  <div class="stats-container flex flex-row w-full gap-2">
    <div class="stats-card p-10 flex flex-row gap-4 items-center">
        <p class="text-white text-md font-bold">{{ stats.totalBlockedAttacks.count }}</p>
        <div>
            <p class="text-white text-sm">Total Blocked Attacks</p>
            <p class="badge success">{{ stats.totalBlockedAttacks.percentage }}% less</p>
        </div>
    </div>
    <div class="stats-card p-10 flex flex-row gap-4 items-center">
        <p class="text-white text-md font-bold">{{ stats.activeThreats.count }}</p>
        <div>
            <p class="text-white text-sm">Active Threats</p>
            <p class="badge success">{{ stats.activeThreats.percentage }}% less</p>
        </div>
    </div>
    <div class="stats-card p-10 flex flex-row gap-4 items-center">
        <p class="text-white text-md font-bold">{{ stats.effectivenessScore.count }}</p>
        <div>
            <p class="text-white text-sm">Effectiveness Score</p>
            <p class="badge success">{{ stats.effectivenessScore.percentage }}% less</p>
        </div>
    </div>
  </div>
</template>

<style scoped>
    .stats-card {
        width: 100%;
        background-color: #000;
    }

    .stats-container .badge {
        margin-top: 4px;
    }
</style>