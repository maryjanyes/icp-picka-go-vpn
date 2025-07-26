<script setup>
    const props = defineProps({
        connect: Function,
    });
</script>

<template>
  <header>
    <div class="header-row px-4 py-6">
        <div class="header-logo-text">
            <img src="/logo.png" alt="PickaGoVPN Logo" />
            <span>Picka Go VPN</span>
        </div>
        <button @click="props.connect" class="connect-button">Connect wallet</button>
    </div>
  </header>
</template>

<style scoped>
    .connect-button {
        background-color: #7C32F3;
        padding: 4px 28px;
        max-height: 42px;
        border-radius: 12px;
        border: none;
        color: #fff;
        font-size: 14px;
        box-shadow: 1px solid #969696;
        letter-spacing: 1px;
        transition: 0.5s background-color;

        &:hover {
            background-color: #b294e2; 
        }
    }

    .header-row {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    .header-logo-text {
        color: #fff;
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 16px;
        font-size: 20px;
    }

    .header-logo-text img {
        width: 55px;
        height: 55px;
    }
</style>