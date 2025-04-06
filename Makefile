.PHONY: kind-create kind-delete kind-logs kind-status

# Create a new Kind cluster
kind-create:
	@echo "Creating Kind cluster..."
	kind create cluster --name k8srs

# Delete the Kind cluster
kind-delete:
	@echo "Deleting Kind cluster..."
	kind delete cluster --name k8srs

# Show cluster logs
kind-logs:
	@echo "Showing Kind cluster logs..."
	kind export logs --name k8srs

# Show cluster status
kind-status:
	@echo "Showing Kind cluster status..."
	kubectl cluster-info --context kind-k8srs
	kubectl get nodes 