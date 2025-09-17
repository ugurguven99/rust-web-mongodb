function showModal(message, modalId) {
    if (modalId === 'successModal') {
        document.getElementById('successModalBody').textContent = message;
    }
    if (modalId === 'errorModal') {
        document.getElementById('errorModalBody').textContent = message;
    }
    $(`#${modalId}`).modal('show');
}
