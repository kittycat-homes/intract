<template>
  <div class="spinner-container">
    <div class="spinner"></div>
  </div>
</template>

<style scoped>
.spinner {
  border-style: solid;
  padding: var(--pad-size-small);
  width: var(--fs-xl);
  height: var(--fs-xl);
  border-width: var(--fs-xl);
  border-color: var(--secondary-accent);
  border-top-color: var(--primary-accent);
  border-left-color: var(--tertiary-accent);
  border-radius: 100%;
  border-bottom-color: var(--quaternary--accent);
  animation: spin 6s linear infinite;
}

.spinner-container {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

@keyframes spin {
  0% {
    transform: rotate(0deg) scale(1);
  }
  50% {
    transform: rotate(1080) scale(2);
  }
  100% {
    transform: rotate(2160deg) scale(1);
  }
}
</style>
