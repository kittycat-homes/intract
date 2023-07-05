<template>
  <div class="spinner-container">
    <div class="spinner"></div>
  </div>
</template>

<style scoped>
.spinner {
  border-style: solid;
  border-width: var(--fs-small);
  height: var(--fs-xl);
  width: var(--fs-xl);
  border-color: var(--secondary-accent);
  border-top-color: var(--primary-accent);
  border-right-color: var(--primary-accent);
  border-radius: var(--fs-xl);
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
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(2160deg);
  }
}
</style>
